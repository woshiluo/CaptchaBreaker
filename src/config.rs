use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path};
use ort::session::Session;
use crate::model::{ModelEnum};


#[derive(Default,Debug)]
pub struct DefaultLoader;
impl ModelLoader for DefaultLoader {}

pub trait ModelLoader: Sync + Send + Debug {
    fn load(&self, model_enum: ModelEnum) -> Result<Session, Box<dyn Error>> {

        let mut model_path =  env::current_dir()?.join("captcha_models");
        let mut url = "";
        match model_enum {
            ModelEnum::Yolo11n => {
                model_path = model_path.join("yolov11n_captcha.onnx");
                url = "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx";
            }
            ModelEnum::Siamese => {
                model_path = model_path.join("siamese.onnx");
                url = "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/siamese.onnx";
            }
        }
        self.build(model_path.to_str().unwrap(), url)
    }
    fn build(&self, save_path: &str, download_url: &str) -> Result<Session, Box<dyn Error>> {
        let model_path = Path::new(save_path);
        if model_path.exists() {
            let session = Session::builder()?.commit_from_file(model_path)?;
            return Ok( session );
        }
        let model_bytes = reqwest::blocking::get(download_url)?.bytes()?;
        // 保存文件
        File::create(model_path)?.write_all(model_bytes.as_ref())?;
        let model =  self.build_from_memory(model_bytes.as_ref());
        Ok( model? )

    }
    /// 直接从内存中构建模型
    fn build_from_memory(&self, model_bytes: &[u8]) -> Result<Session, Box<dyn Error>> {
        let session = Session::builder()?.commit_from_memory(model_bytes)?;
        Ok( session )
    }
}