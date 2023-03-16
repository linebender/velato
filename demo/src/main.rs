// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

use std::{fs, time::Instant};

use anyhow::Result;
use clap::Parser;
use velato::Composition;
use vello::RendererOptions;
use vello::{
    block_on_wgpu,
    kurbo::{Affine, Vec2},
    peniko::Color,
    util::RenderContext,
    RenderParams, Renderer, Scene, SceneBuilder,
};

use winit::{event_loop::EventLoop, window::Window};

#[derive(Parser, Debug)]
#[command(about, long_about = None, bin_name="cargo run -p with_winit --")]
struct Args {
    /// Path to the svg file to render. If not set, the GhostScript Tiger will be rendered
    #[cfg(not(target_arch = "wasm32"))]
    file: std::path::PathBuf,
    /// When rendering an svg, what scale to use
    #[arg(long)]
    scale: Option<f64>,
}

async fn run(event_loop: EventLoop<()>, window: Window, args: Args, composition: Composition) {
    use winit::{event::*, event_loop::ControlFlow};
    let mut render_cx = RenderContext::new().unwrap();
    let size = window.inner_size();
    let mut surface = render_cx
        .create_surface(&window, size.width, size.height)
        .await;
    let device_handle = &render_cx.devices[surface.dev_id];
    let mut velato_renderer = velato::Renderer::new();
    let mut renderer = Renderer::new(
        &device_handle.device,
        &RendererOptions {
            surface_format: Some(surface.format),
        },
    )
    .unwrap();
    let mut scene = Scene::new();
    let start = Instant::now();

    let mut transform = Affine::scale(args.scale.unwrap_or(1.0));
    let mut mouse_down = false;
    let mut prior_position: Option<Vec2> = None;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state == ElementState::Pressed {
                    match input.virtual_keycode {
                        Some(VirtualKeyCode::Escape) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::Resized(size) => {
                render_cx.resize_surface(&mut surface, size.width, size.height);
                window.request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == &MouseButton::Left {
                    mouse_down = state == &ElementState::Pressed;
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                const BASE: f64 = 1.05;
                const PIXELS_PER_LINE: f64 = 20.0;

                if let Some(prior_position) = prior_position {
                    let exponent = if let MouseScrollDelta::PixelDelta(delta) = delta {
                        delta.y / PIXELS_PER_LINE
                    } else if let MouseScrollDelta::LineDelta(_, y) = delta {
                        *y as f64
                    } else {
                        0.0
                    };
                    transform = Affine::translate(prior_position)
                        * Affine::scale(BASE.powf(exponent))
                        * Affine::translate(-prior_position)
                        * transform;
                } else {
                    eprintln!("Scrolling without mouse in window; this shouldn't be possible");
                }
            }
            WindowEvent::CursorLeft { .. } => {
                prior_position = None;
            }
            WindowEvent::CursorMoved { position, .. } => {
                let position = Vec2::new(position.x, position.y);
                if mouse_down {
                    if let Some(prior) = prior_position {
                        transform = Affine::translate(position - prior) * transform;
                    }
                }
                prior_position = Some(position);
            }
            _ => {}
        },
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let width = surface.config.width;
            let height = surface.config.height;
            let device_handle = &render_cx.devices[surface.dev_id];
            let time = start.elapsed().as_secs_f32();

            let mut builder = SceneBuilder::for_scene(&mut scene);
            velato_renderer.render(&composition, time, transform, 1.0, &mut builder);

            let surface_texture = surface
                .surface
                .get_current_texture()
                .expect("failed to get surface texture");
            block_on_wgpu(
                &device_handle.device,
                renderer.render_to_surface_async(
                    &device_handle.device,
                    &device_handle.queue,
                    &scene,
                    &surface_texture,
                    &RenderParams {
                        base_color: Color::BLACK,
                        width,
                        height,
                    },
                ),
            )
            .expect("failed to render to surface");
            surface_texture.present();
            device_handle.device.poll(wgpu::Maintain::Poll);
        }
        _ => {}
    });
}

fn main() -> Result<()> {
    // TODO: initializing both env_logger and console_logger fails on wasm.
    // Figure out a more principled approach.
    env_logger::init();
    let args = Args::parse();
    use winit::{dpi::LogicalSize, window::WindowBuilder};
    let event_loop = EventLoop::new();
    let contents = fs::read(&args.file)?;
    // TODO: Implement proper error handling in velato so that anyhow can be properly used here
    let composition = velato::Composition::from_bytes(&contents).unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title("Velato demo")
        .build(&event_loop)
        .unwrap();
    pollster::block_on(run(event_loop, window, args, composition));
    Ok(())
}
