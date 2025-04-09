use std::error::Error;
use ort::session::Session;
use crate::model::Model;
use std::{env, fs};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use ort::execution_providers::CUDAExecutionProvider;
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
        let client = reqwest::blocking::Client::builder().user_agent("CaptchaBreaker").build()?;
        let bytes = client.get(url.as_str()).send()?.bytes()?;
        let mut file = File::create(path)?;
        file.write_all(bytes.as_ref())?;
        Ok(Session::builder()?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_memory(bytes.as_ref())?)
    } else {
        let bytes = fs::read(path)?;
        Ok(Session::builder()?
            .with_execution_providers([CUDAExecutionProvider::default().build()])?
            .commit_from_memory(bytes.as_ref())?)
    }
}
impl ModelLoaderTrait for DefaultModelLoader {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>> {
        let root = env::current_dir()?;
        let model_root = root.join("models");
        if !model_root.exists() {
            fs::create_dir_all(&model_root)?;
        }

        match model {
            Model::Yolo11n => {
                load_one_model(
                    model_root.join("yolov11n_captcha.onnx").as_path(),
                    &Url::parse(
                        "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx",
                    )?,
                )
            }
            Model::Siamese => {
                load_one_model(
                    model_root.join("siamese.onnx").as_path(),
                    &Url::parse(
                        "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/siamese.onnx",
                    )?,
                )
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