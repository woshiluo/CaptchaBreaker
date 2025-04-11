use crate::model::Model;
use ort::execution_providers::{CPUExecutionProvider, ExecutionProviderDispatch};
use ort::session::Session;
use reqwest::Url;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, fs};
use image::EncodableLayout;


pub enum ModelLoader {
    DefaultModelLoader,
    CustomModelLoader(Box<dyn ModelLoaderTrait>),
}
pub trait ModelLoaderTrait {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>> {
        self.load_with_execution_providers(model, vec![CPUExecutionProvider::default().build()])
    }
    fn load_with_execution_providers(&self, model: Model, providers: Vec<ExecutionProviderDispatch>) -> Result<Session, Box<dyn Error>>;
}

#[derive(Default)]
pub struct DefaultModelLoader;

fn load_one_model(path: &Path, url: &Url, providers: impl IntoIterator<Item = ExecutionProviderDispatch>) -> Result<Session, Box<dyn Error>> {
    let bytes;
    if !path.exists() {
        let client = reqwest::blocking::Client::builder()
            .user_agent("CaptchaBreaker")
            .build()?;
        bytes = client.get(url.as_str()).send()?.bytes()?.as_bytes().to_owned();
        let mut file = File::create(path)?;
        file.write_all(bytes.as_ref())?;

    } else {
        bytes = fs::read(path)?;
    }
    Ok(Session::builder()?
        .with_execution_providers(providers)?
        .commit_from_memory(bytes.as_ref())?)
}
impl ModelLoaderTrait for DefaultModelLoader {
    fn load_with_execution_providers(&self, model: Model, providers: Vec<ExecutionProviderDispatch>) -> Result<Session, Box<dyn Error>> {
        let root = env::current_dir()?;
        let model_root = root.join("models");
        if !model_root.exists() {
            fs::create_dir_all(&model_root)?;
        }
        match model {
            Model::Yolo11n => load_one_model(
                model_root.join("yolov11n_captcha.onnx").as_path(),
                &Url::parse(
                    "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx",
                )?,
                providers,
            ),
            Model::Siamese => load_one_model(
                model_root.join("siamese.onnx").as_path(),
                &Url::parse(
                    "https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/siamese.onnx",
                )?,
                providers,
            ),
        }
    }
}

impl ModelLoader {
    pub(crate) fn get_model_loader(self) -> Box<dyn ModelLoaderTrait> {
        match self {
            ModelLoader::DefaultModelLoader => Box::new(DefaultModelLoader::default()),
            ModelLoader::CustomModelLoader(model_loader) => model_loader,
        }
    }
}
