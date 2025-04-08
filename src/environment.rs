use std::collections::{HashMap};
use std::error::Error;
use ort::session::Session;
use crate::captcha::{CaptchaBreaker, CaptchaBreakerTrait};
use crate::loader::{ModelLoader, ModelLoaderTrait};
use crate::model::Model;

pub struct CaptchaEnvironment {
    model_loader: Box<dyn ModelLoaderTrait>,
    models: HashMap<Model, Session>,
}

impl Default for CaptchaEnvironment {
    fn default() -> Self {
        CaptchaEnvironment {
            model_loader: ModelLoader::DefaultModelLoader.get_model_loader(),
            models: HashMap::default(),
        }
    }
}



impl CaptchaEnvironment {
    pub fn with_model_loader(model_loader: ModelLoader) -> Self {
        CaptchaEnvironment {
            model_loader: model_loader.get_model_loader(),
            models: HashMap::default(),
        }
    }

    pub fn load_captcha_breaker(&mut self, cb: CaptchaBreaker) -> Result<impl CaptchaBreakerTrait, Box<dyn Error>> {
        cb
    }

    pub(crate) fn load_model(&mut self, model: Model) -> Result<&Session, Box<dyn Error>> {
        let session = self.models.entry(model).or_insert_with(|| {
            self.model_loader.load(model).unwrap() // 你可以调整错误处理
        });
        Ok(&*session)
    }

    pub(crate) fn load_models(&mut self, models: Vec<Model>) -> Result<Vec<&Session>, Box<dyn Error>> {
        for model in &models {
            self.models.entry(*model).or_insert_with(|| {
                self.model_loader.load(*model).unwrap()
            });
        }
        let mut sessions: Vec<&Session> = vec![];
        for model in &models {
            let session = self.models.get(model).unwrap();
            sessions.push(session);
        }
        Ok(sessions)
    }
}