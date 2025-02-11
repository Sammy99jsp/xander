//! Visualizations

use std::{
    fmt::Write,
    fs,
    mem::{self},
    ops::Deref,
    slice,
};

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use itertools::Itertools;
use skia::{Color4f, Paint, Rect};

use crate::core::geom::P3;

pub mod color;
pub mod rich;

pub trait Drawable {
    fn draw(&self, canvas: &skia::Canvas);
}

pub struct Root {
    width: u32,
    height: u32,
    border: u32,
    scale: u32,
    children: Vec<Box<dyn Drawable>>,
}

impl Root {
    pub fn new(width: u32, height: u32, scale: u32, border: u32) -> Self {
        Self {
            width,
            height,
            scale,
            border,
            children: Vec::new(),
        }
    }

    pub fn add_child<R: Drawable + 'static>(&mut self, child: R) -> &mut Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn render(&self) -> Image {
        let mut surface = skia::surface::surfaces::raster_n32_premul((
            self.scale as i32 * self.width as i32 + 2 * self.border as i32,
            self.scale as i32 * self.height as i32 + 2 * self.border as i32,
        ))
        .unwrap();

        let canvas = surface.canvas();

        // Draw outline...

        let outline = Paint::new(color::OUTLINE, None);
        let outlined = Rect::new(
            0.0,
            0.0,
            self.width as f32 * self.scale as f32 + 2.0 * self.border as f32,
            self.height as f32 * self.scale as f32 + 2.0 * self.border as f32,
        );
        canvas.draw_rect(outlined, &outline);

        canvas.translate((self.border as f32, self.border as f32));
        canvas.scale((self.scale as f32, self.scale as f32));

        // Draw children.
        self.children.iter().for_each(|child| child.draw(canvas));

        // Encode to PNG
        Image(
            surface
                .image_snapshot()
                .encode(None, skia::EncodedImageFormat::PNG, None)
                .expect("Valid encode"),
        )
    }
}

pub struct FindReplace {
    pub find: Color4f,
    pub replace: Color4f,
}

pub struct Token {
    pos: P3,
    icon: skia::Image,
    filter: skia::ColorFilter,
}

impl Token {
    pub fn new(pos: P3, icon: skia::Image, colors: FindReplace) -> Self {
        let uniforms = unsafe {
            let ptr = &raw const colors as *const u8;
            assert!(ptr.is_aligned_to(1));

            // SAFETY: We should be aligned, we don't overrun, etc.
            let data = slice::from_raw_parts(ptr, mem::size_of::<FindReplace>());
            skia::Data::new_copy(data)
        };

        let effect =
            skia::RuntimeEffect::make_for_color_filer(include_str!("./find_replace.sksl"), None)
                .expect("Valid effect.");

        // let img = skia::Image::from_encoded(data);
        let filter = effect
            .make_color_filter(uniforms, None)
            .expect("Valid filter.");

        Self { pos, icon, filter }
    }

    pub fn draw(&self, canvas: &skia::Canvas, rect: skia::Rect) {
        let mut paint = skia::Paint::default();
        paint.set_color_filter(self.filter.clone());

        canvas.draw_image_rect(&self.icon, None, rect, &paint);
    }
}

pub struct Grid {
    /// Length of the square's edges (in px)
    pub square_len: u32,

    /// Width (in px)
    pub width: u32,

    /// Height (in px)
    pub height: u32,

    pub tokens: Vec<Token>,
}

impl Grid {
    pub fn square_floor(&self, pos: P3) -> (f32, f32) {
        let sq = self.square_len as f32;
        (pos.x.div_euclid(sq) * sq, pos.y.div_euclid(sq) * sq)
    }
}

impl Drawable for Grid {
    fn draw(&self, canvas: &skia::Canvas) {
        let sq_light = Paint::new(color::LIGHT_SQUARE, None);
        let sq_dark = Paint::new(color::DARK_SQUARE, None);

        // Draw squares.
        for (x, y) in (0..self.width.div_euclid(self.square_len))
            .cartesian_product(0..self.height.div_ceil(self.square_len))
        {
            let color = if (x + y) % 2 == 0 {
                &sq_light
            } else {
                &sq_dark
            };
            let (x, y) = (
                x as f32 * self.square_len as f32,
                y as f32 * self.square_len as f32,
            );

            canvas.draw_rect(
                Rect::new(
                    x,
                    self.height as f32 - (y + self.square_len as f32),
                    x + self.square_len as f32,
                    self.height as f32 - y,
                ),
                color,
            );
        }

        for token in &self.tokens {
            let (x, y) = self.square_floor(token.pos);
            let rect = skia::Rect::new(
                x,
                self.height as f32 - (y + self.square_len as f32),
                x + self.square_len as f32,
                self.height as f32 - y,
            );
            token.draw(canvas, rect);
        }
    }
}

pub struct Image(skia::Data);

fn len_b64(by: &[u8]) -> usize {
    4 * by.len().div_ceil(3)
}

impl Image {
    pub fn as_html(&self) -> String {
        fs::write("temp.png", self.0.deref()).expect("Write temp file.");

        const START: &str =
            r#"<div style="width: 50vw; margin: auto;"><img src="data:image/png;base64,"#;
        const END: &str = r#"" alt="Xander Grid Preview"></div>"#;

        let mut s = String::with_capacity(len_b64(&self.0) + START.len() + END.len());

        s.write_str(START).unwrap();
        B64.encode_string(self.0.deref(), &mut s);
        s.write_str(END).unwrap();

        s
    }
}
