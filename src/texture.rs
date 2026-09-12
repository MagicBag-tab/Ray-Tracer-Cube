use image::{DynamicImage, GenericImageView};
use crate::color::Color;

pub struct Texture {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Texture {
    pub fn new(file_path: &str) -> Self {
        let img = image::open(file_path).expect(&format!("Failed to load texture: {}", file_path));
        let (width, height) = img.dimensions();
        Texture {
            image: img,
            width,
            height,
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        // Clamp UV coordinates to [0.0, 1.0] to prevent out-of-bounds errors
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Convert UV to pixel coordinates
        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = ((1.0 - v) * (self.height - 1) as f32).round() as u32; // Invert V so 0.0 is bottom

        let pixel = self.image.get_pixel(x, y);
        let rgba = pixel.0;

        Color::new(rgba[0], rgba[1], rgba[2])
    }
}
