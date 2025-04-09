use std::error::Error;
use ort::session::Session;
use crate::model::Model;
use std::{env, fs};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use reqwest::Url;

pub enum ModelLoader {
    DefaultModelLoader,
    CustomModelLoader(Box<dyn ModelLoaderTrait>),
}
pub trait ModelLoaderTrait {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>>;
}

#[derive(Default)]
struct DefaultModelLoader;

fn load_one_model(path: &Path, url: &Url) -> Result<Session, Box<dyn Error>> {
    if !path.exists() {
        let bytes = reqwest::blocking::get(url.as_str())?.bytes()?;
        let mut file = File::create(path)?;
        file.write_all(bytes.as_ref())?;
        Ok(Session::builder().unwrap().commit_from_memory(bytes.as_ref())?)
    } else {
        let bytes = fs::read(path)?;
        Ok(Session::builder().unwrap().commit_from_memory(bytes.as_ref())?)
    }
}
impl ModelLoaderTrait for DefaultModelLoader {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>> {
        let root = env::current_dir().unwrap();
        let model_root = root.join("models");

        match model {
            Model::Yolo11n => {
                load_one_model(model_root.join("yolov11n_captcha.onnx").as_path(),
                               &Url::parse("https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx")?)
            }
            Model::Siamese => {
                load_one_model(model_root.join("siamese.onnx").as_path(),
                               &Url::parse("https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/siamese.onnx")?)
            }
        }
    }
}

impl ModelLoader {
    pub(crate) fn get_model_loader(self) -> Box<dyn ModelLoaderTrait> {
        match self {
            ModelLoader::DefaultModelLoader => {
                Box::new(DefaultModelLoader::default())
            }
            ModelLoader::CustomModelLoader(model_loader) => {
                model_loader
            }
        }
    }
}