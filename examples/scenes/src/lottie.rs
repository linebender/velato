// Copyright 2022 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::SceneParams;
#[cfg(not(target_arch = "wasm32"))]
use crate::{ExampleScene, SceneSet};
#[cfg(not(target_arch = "wasm32"))]
use anyhow::{Ok, Result};
use instant::Instant;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use velato::Composition;
use vello::kurbo::{Affine, Vec2};
use vello::Scene;

#[cfg(not(target_arch = "wasm32"))]
pub fn scene_from_files(files: &[PathBuf]) -> Result<SceneSet> {
    scene_from_files_inner(files, || ())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn default_scene(command: impl FnOnce() -> clap::Command) -> Result<SceneSet> {
    let assets_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../assets/")
        .canonicalize()?;
    let mut has_empty_directory = false;
    let result = scene_from_files_inner(&[assets_dir.join("Tiger.json")], || {
        has_empty_directory = true
    })?;
    if has_empty_directory {
        let mut command = command();
        command.build();
        panic!("No test files are available.");
    }
    Ok(result)
}

#[cfg(not(target_arch = "wasm32"))]
fn scene_from_files_inner(
    files: &[PathBuf],
    mut empty_dir: impl FnMut(),
) -> std::result::Result<SceneSet, anyhow::Error> {
    let mut scenes = Vec::new();
    for path in files {
        if path.is_dir() {
            let mut count = 0;
            let start_index = scenes.len();
            for file in read_dir(path)? {
                let entry = file?;
                if let Some(extension) = Path::new(&entry.file_name()).extension() {
                    if extension == "json" {
                        count += 1;
                        scenes.push(example_scene_of(entry.path()));
                    }
                }
            }
            // Ensure a consistent order within directories
            scenes[start_index..].sort_by_key(|scene| scene.config.name.to_lowercase());
            if count == 0 {
                empty_dir();
            }
        } else {
            scenes.push(example_scene_of(path.to_owned()));
        }
    }
    Ok(SceneSet { scenes })
}

#[cfg(not(target_arch = "wasm32"))]
fn example_scene_of(file: PathBuf) -> ExampleScene {
    let name = file
        .file_stem()
        .map(|it| it.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    ExampleScene {
        function: Box::new(lottie_function_of(name.clone(), move || {
            std::fs::read_to_string(&file)
                .unwrap_or_else(|e| panic!("failed to read lottie file {file:?}: {e}"))
        })),
        config: crate::SceneConfig {
            animated: true,
            name,
        },
    }
}

pub fn lottie_function_of<R: AsRef<str>>(
    name: String,
    contents: impl FnOnce() -> R + Send + 'static,
) -> impl FnMut(&mut Scene, &mut SceneParams) {
    let start = Instant::now();
    let lottie = Arc::new(
        velato::Composition::from_bytes(contents().as_ref().as_bytes())
            .unwrap_or_else(|e| panic!("failed to parse lottie file {name}: {e}")),
    );
    eprintln!("Parsed lottie {name} in {:?}", start.elapsed());
    fn render_lottie_contents(lottie: &Composition, start: &Instant) -> (Scene, Vec2) {
        let mut new_scene = Scene::new();
        let frame = ((start.elapsed().as_secs_f32() * lottie.frame_rate)
            % (lottie.frames.end - lottie.frames.start))
            + lottie.frames.start;
        velato::Renderer::new().render(lottie, frame, Affine::IDENTITY, 1.0, &mut new_scene);
        let resolution = Vec2::new(lottie.width as f64, lottie.height as f64);
        (new_scene, resolution)
    }

    let started = instant::Instant::now();
    let lottie = lottie.clone();
    move |scene, params| {
        let (scene_frag, resolution) = render_lottie_contents(&lottie, &started);
        scene.append(&scene_frag, None);
        params.resolution = Some(resolution);
    }
}
