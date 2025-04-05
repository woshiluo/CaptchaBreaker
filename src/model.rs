use std::vec;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use ort::{inputs, session::Session};
use ndarray::{s, Array4, Axis};
use crate::config::{GlobalConfig, ModelConfig};

pub struct OnnxModel {
    pub session: Session,
}

impl OnnxModel {
    /// 从配置中构建模型(文件 | 没有文件时下载)
    async fn build_from_config_async(model_config: &ModelConfig) -> Result<OnnxModel, Box<dyn Error>> {
        let model_path = Path::new(&model_config.file_path);
        if model_path.exists() {
            let session = Session::builder()?.commit_from_file(model_path)?;
            return Ok(OnnxModel { session });
        }
        let model_bytes = reqwest::get(&model_config.download_url).await?.bytes().await?;
        // 保存文件
        File::create(model_path)?.write_all(model_bytes.as_ref())?;
        let model =  Self::build_from_memory(model_bytes.as_ref());
        Ok( model? )

    }
    /// 直接从内存中构建模型
    fn build_from_memory(model_bytes: &[u8]) -> Result<OnnxModel, Box<dyn Error>> {
        let session = Session::builder()?.commit_from_memory(model_bytes)?;
        Ok( OnnxModel { session } )
    }
}


#[cfg(feature = "_model_yolo11n")]
pub struct Yolo11nModel {
    model: OnnxModel,
}

impl Yolo11nModel {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let model = OnnxModel::build_from_config_async(&GlobalConfig::instance().models_config.yolo11n).await?;
        Ok( Yolo11nModel { model } )
    }

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

        let outputs = &self.model.session.run(
            inputs!["images" => input].unwrap()
        ).unwrap();

        let output = outputs["output0"].try_extract_tensor::<f32>().unwrap();
        let mut boxes: Vec<f32> = Vec::new();
        let output = output.slice(s![.., .., 0]);
        for row in output.axis_iter(Axis(0)) {
                
        }
    }
}