use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use ndarray::{s, Array4, Axis};
use ort::{inputs, session::Session};

#[derive(Debug)]
pub enum ModelEnum {
    Yolo11n,
    Siamese,
}

#[derive(Debug)]
pub struct Yolo11nModel {
    session: Session,
}

impl Yolo11nModel {
    fn run(&self, image: DynamicImage) {
        let (width, height) = (image.width(), image.height());
        let mut new_image = ImageBuffer::from_pixel(384, 384,  Rgba([0u8, 0u8, 0u8, 255u8]));

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                new_image.put_pixel(x, y, pixel);
            }
        }

        let mut input = Array4::<f32>::zeros((1, 3, 384, 384));
        for x in 0..384 {
            for y in 0..384 {
                let pixel = new_image.get_pixel(x, y);
                input[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                input[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                input[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
            }
        }

        let outputs = &self.session.run(
            inputs!["images" => input].unwrap()
        ).unwrap();

        let output = outputs["output0"].try_extract_tensor::<f32>().unwrap();
        let boxes: Vec<f32> = Vec::new();
        let output = output.slice(s![.., .., 0]);
        for row in output.axis_iter(Axis(0)) {

        }
    }
}

pub struct SiameseModel {}

