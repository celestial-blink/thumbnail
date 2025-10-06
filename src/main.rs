use image::{ImageFormat, ImageResult, Rgb, RgbImage, imageops::FilterType::Triangle};
use std::env;

struct TImage {
    input_image_path: String,
    output_image_path: String,
    output_format: String,
    padding: u16,
}

impl TImage {
    fn new(
        input_image_path: String,
        output_image_path: String,
        output_format: String,
        padding: u16,
    ) -> Self {
        Self {
            input_image_path,
            output_image_path,
            output_format,
            padding,
        }
    }

    fn resize(&self, width: u16, height: u16) -> ImageResult<()> {
        let mut background_image = RgbImage::new(width.into(), height.into());
        for pixel in background_image.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }

        let input_image_file = image::open(&self.input_image_path)?;
        let output_width: u32 = (width - self.padding).into();
        let output_height: u32 = (height - self.padding).into();
        let output_image_file = input_image_file.resize(output_width, output_height, Triangle);

        let y = (height as i64) / 2 - (output_image_file.height() as i64 / 2);
        let x = (width as i64) / 2 - (output_image_file.width() as i64 / 2);

        image::imageops::overlay(&mut background_image, &output_image_file.to_rgb8(), x, y);

        let image_format = if self.output_format == "png" {
            ImageFormat::Png
        } else {
            ImageFormat::Jpeg
        };

        let _ = background_image.save_with_format(&self.output_image_path, image_format);

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // required 6 arguments
    /*
    - input image path
    - output image path
    - format image "png|jpg"
    - padding number value
    - width image number value
    - heigh image number value
     */

    if let Ok(
        [
            _,
            input_image_path,
            output_image_path,
            format,
            padding,
            width,
            height,
        ],
    ) = <[String; 7]>::try_from(args)
    {
        // let input_image_path = String::from("./logo.png");
        // let output_image_path = String::from("./test/takes.png");
        // let format = String::from("png");
        // let padding: u16 = 6;
        let padding: u16 = padding.parse().unwrap_or(6);
        let t_image = TImage::new(input_image_path, output_image_path, format, padding);
        let width: u16 = width.parse().unwrap_or(150);
        let height: u16 = height.parse().unwrap_or(150);
        let _ = t_image.resize(width, height);
        println!("[ok]");
    } else {
        println!("[error] Required 6 arguments");
    }
}
