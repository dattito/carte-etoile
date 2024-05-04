use std::io::Cursor;

use image::{imageops::overlay, DynamicImage, ImageResult};

static POINT_HEIGHT_RATIO: f32 = 0.40;
static OPACITY_DISABLED: f32 = 0.20;

#[derive(Debug)]
pub struct ImageMaker {
    background_image: DynamicImage,
    point_image: DynamicImage,
}

impl ImageMaker {
    pub fn new(background_image_path: &str, point_image_path: &str) -> ImageResult<Self> {
        Ok(Self {
            background_image: image::open(background_image_path)?,
            point_image: image::open(point_image_path)?,
        })
    }

    pub fn generate_points_image(
        &self,
        total_points: u32,
        current_points: u32,
    ) -> ImageResult<Vec<u8>> {
        let mut background_image = self.background_image.clone().to_rgba8();

        let (img_width, img_height) = background_image.dimensions();
        let point_height = (img_height as f32 * POINT_HEIGHT_RATIO).round() as u32;
        let space_height =
            (img_height as f32 * (1.0 - 2.0 * POINT_HEIGHT_RATIO) / 3.0).round() as u32;

        let num_in_row = total_points / 2;
        let point_width = (img_width as f32 - (num_in_row as f32 + 1.0) * space_height as f32)
            / num_in_row as f32;
        let point_enabled = self
            .point_image
            .resize(
                point_width as u32,
                point_height,
                image::imageops::FilterType::Nearest,
            )
            .to_rgba8();

        let mut point_disabled = point_enabled.clone();

        point_disabled
            .pixels_mut()
            .for_each(|p| p.0[3] = (p.0[3] as f32 * OPACITY_DISABLED) as u8);

        let positions = self.calculate_positions(
            img_height,
            point_width as u32,
            point_height,
            space_height,
            total_points,
        );

        positions.into_iter().enumerate().for_each(|(i, (x, y))| {
            overlay(
                &mut background_image,
                if i < current_points.try_into().unwrap() {
                    &point_enabled
                } else {
                    &point_disabled
                },
                x.into(),
                y.into(),
            );
        });

        let mut buf = Vec::new();
        background_image.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;

        Ok(buf)
    }

    fn calculate_positions(
        &self,
        img_height: u32,
        point_width: u32,
        point_height: u32,
        space_height: u32,
        num_points: u32,
    ) -> Vec<(u32, u32)> {
        let mut positions = Vec::new();
        let num_in_row = num_points / 2;
        let horizontal_spacing = space_height; // Equal to vertical spacing

        let y_top = space_height; // Top margin
        let y_bottom = img_height - point_height - space_height; // Bottom margin

        for row in 0..2 {
            let y = if row == 0 { y_top } else { y_bottom };
            for n in 0..num_in_row {
                let x = horizontal_spacing + (point_width + horizontal_spacing) * n;
                positions.push((x, y));
            }
        }
        positions
    }
}
