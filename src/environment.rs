use crate::captcha::CaptchaBreaker;
use crate::loader::{ModelLoader, ModelLoaderTrait};
use crate::model::Model;
use ort::session::Session;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use ort::execution_providers::{CPUExecutionProvider, ExecutionProviderDispatch};

pub struct CaptchaEnvironment {
    model_loader: Box<dyn ModelLoaderTrait>,
    models: RefCell<HashMap<Model, Rc<Session>>>,
    ep: Vec<ExecutionProviderDispatch>,
}

impl Default for CaptchaEnvironment {
    fn default() -> Self {
        CaptchaEnvironment {
            model_loader: ModelLoader::DefaultModelLoader.get_model_loader(),
            models: Default::default(),
            ep: vec![CPUExecutionProvider::default().build()],
        }
    }
}

impl CaptchaEnvironment {
    pub fn with_model_loader(model_loader: ModelLoader) -> Self {
        CaptchaEnvironment {
            model_loader: model_loader.get_model_loader(),
            models: Default::default(),
            ep: vec![CPUExecutionProvider::default().build()],
        }
    }

    pub fn with_ep(ep: Vec<ExecutionProviderDispatch>) -> Self {
        CaptchaEnvironment {
            model_loader: ModelLoader::DefaultModelLoader.get_model_loader(),
            models: Default::default(),
            ep,
        }
    }

    pub fn with_model_loader_and_ep(model_loader: Box<dyn ModelLoaderTrait>, ep: Vec<ExecutionProviderDispatch>) -> Self {
        CaptchaEnvironment {
            model_loader,
            models: Default::default(),
            ep,
        }
    }

    pub fn load_captcha_breaker<CB>(&self) -> Result<CB, Box<dyn Error>>
    where
        CB: CaptchaBreaker,
    {
        CB::build(self)
    }

    pub(crate) fn load_models(
        &self,
        models: Vec<Model>,
    ) -> Result<Vec<Rc<Session>>, Box<dyn Error>> {
        let mut res = vec![];
        for model in models {
            res.push(self.load_one_model(model)?);
        }
        Ok(res)
    }

    fn load_one_model(&self, model: Model) -> Result<Rc<Session>, Box<dyn Error>> {
        // 检查模型是否已加载
        if let Some(session) = self.models.borrow().get(&model) {
            return Ok(Rc::clone(session));
        }

        let session = self.model_loader.load_with_execution_providers(model, self.ep.clone())?;
        let session_rc = Rc::new(session);
        self.models
            .borrow_mut()
            .insert(model, Rc::clone(&session_rc));

        Ok(session_rc)
    }
}
