// Copyright 2022 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use skrifa::MetadataProvider;
use skrifa::raw::FontRef;
use std::sync::Arc;
use vello::kurbo::Affine;
use vello::peniko::{Blob, Brush, BrushRef, Font, StyleRef};
use vello::{Glyph, Scene};

// This is very much a hack to get things working.
// On Windows, can set this to "c:\\Windows\\Fonts\\seguiemj.ttf" to get color
// emoji
const ROBOTO_FONT: &[u8] = include_bytes!("../../assets/roboto/Roboto-Regular.ttf");
pub struct RobotoText {
    font: Font,
}

impl RobotoText {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            font: Font::new(Blob::new(Arc::new(ROBOTO_FONT)), 0),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_run<'a>(
        &mut self,
        scene: &mut Scene,
        font: Option<&Font>,
        size: f32,
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &str,
    ) {
        self.add_var_run(
            scene,
            font,
            size,
            &[],
            brush,
            transform,
            glyph_transform,
            style,
            text,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_var_run<'a>(
        &mut self,
        scene: &mut Scene,
        font: Option<&Font>,
        size: f32,
        variations: &[(&str, f32)],
        brush: impl Into<BrushRef<'a>>,
        transform: Affine,
        glyph_transform: Option<Affine>,
        style: impl Into<StyleRef<'a>>,
        text: &str,
    ) {
        let default_font = &self.font;
        let font = font.unwrap_or(default_font);
        let font_ref = to_font_ref(font).unwrap();
        let brush = brush.into();
        let style = style.into();
        let axes = font_ref.axes();
        let font_size = skrifa::instance::Size::new(size);
        let var_loc = axes.location(variations.iter().copied());
        let charmap = font_ref.charmap();
        let metrics = font_ref.metrics(font_size, &var_loc);
        let line_height = metrics.ascent - metrics.descent + metrics.leading;
        let glyph_metrics = font_ref.glyph_metrics(font_size, &var_loc);
        let mut pen_x = 0_f32;
        let mut pen_y = 0_f32;
        scene
            .draw_glyphs(font)
            .font_size(size)
            .transform(transform)
            .glyph_transform(glyph_transform)
            .normalized_coords(bytemuck::cast_slice(var_loc.coords()))
            .brush(brush)
            .draw(
                style,
                text.chars().filter_map(|ch| {
                    if ch == '\n' {
                        pen_y += line_height;
                        pen_x = 0.0;
                        return None;
                    }
                    let gid = charmap.map(ch).unwrap_or_default();
                    let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                    let x = pen_x;
                    pen_x += advance;
                    Some(Glyph {
                        id: gid.to_u32(),
                        x,
                        y: pen_y,
                    })
                }),
            );
    }

    pub fn add(
        &mut self,
        scene: &mut Scene,
        font: Option<&Font>,
        size: f32,
        brush: Option<&Brush>,
        transform: Affine,
        text: &str,
    ) {
        use vello::peniko::{Color, Fill};
        let brush = brush.unwrap_or(&Brush::Solid(Color::WHITE));
        self.add_run(
            scene,
            font,
            size,
            brush,
            transform,
            None,
            Fill::NonZero,
            text,
        );
    }
}

fn to_font_ref(font: &Font) -> Option<FontRef<'_>> {
    use skrifa::raw::FileRef;
    let file_ref = FileRef::new(font.data.as_ref()).ok()?;
    match file_ref {
        FileRef::Font(font) => Some(font),
        FileRef::Collection(collection) => collection.get(font.index).ok(),
    }
}
