use image::{
    GenericImage, GenericImageView, ImageFormat, ImageResult, Rgb, RgbImage, Rgba,
    imageops::FilterType::Triangle, load_from_memory,
};
use std::{
    env,
    io::{self, Read, Write},
};

struct TImage {
    padding: u16,
}

impl TImage {
    fn new(padding: u16) -> Self {
        Self {
            // output_image_path,            
            padding,
        }
    }

    fn resize(&self, width: u16, height: u16) -> ImageResult<()> {
        let mut background_image = RgbImage::new(width.into(), height.into());
        for pixel in background_image.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }

        let mut input_buffer_image_file = Vec::new();
        let _ = io::stdin().read_to_end(&mut input_buffer_image_file);

        if input_buffer_image_file.is_empty() {
            eprintln!("Error: No se recibieron datos en stdin.");
            return Ok(());
        }

        let mut input_image_file = load_from_memory(&input_buffer_image_file)?;

        let (i_width, i_height) = input_image_file.dimensions();
        for x in 0..i_width {
            for y in 0..i_height {
                let pixel = input_image_file.get_pixel(x, y);
                let [_r, _g, _b, a] = pixel.0;
                if a == 0 {
                    let _ =
                        input_image_file.put_pixel(x.into(), y.into(), Rgba([255, 255, 255, 255]));
                }
            }
        }

        let output_width: u32 = (width - self.padding).into();
        let output_height: u32 = (height - self.padding).into();
        let output_image_file = input_image_file.resize(output_width, output_height, Triangle);

        let y = (height as i64) / 2 - (output_image_file.height() as i64 / 2);
        let x = (width as i64) / 2 - (output_image_file.width() as i64 / 2);

        image::imageops::overlay(&mut background_image, &output_image_file.to_rgb8(), x, y);

        let image_format = ImageFormat::WebP;

        // let _ = background_image.save_with_format(&self.output_image_path, image_format);
        let mut output_image = Vec::new();

        let _ = background_image.write_to(
            &mut io::Cursor::new(&mut output_image),
            image_format);

        let _ = io::stdout().write_all(&output_image)?;
        
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // required 3 arguments
    /*
    - padding number value
    - width image number value
    - heigh image number value
     */

    //< logo.png > ./test/marina_hes.jpg jpg 10 120 100

    if let Ok([padding, width, height]) =
        <[String; 3]>::try_from(args)
    {
        // let output_image_path = String::from("./test/takes.png");
        // let format = String::from("png");
        // let padding: u16 = 6;
        let padding: u16 = padding.parse().unwrap_or(6);
        let t_image = TImage::new(padding);
        let width: u16 = width.parse().unwrap_or(150);
        let height: u16 = height.parse().unwrap_or(150);
        let _ = t_image.resize(width, height);
        eprintln!("[ok]");
    } else {
        eprintln!("[error] Required 3 arguments");
    }
}
