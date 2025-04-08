use std::error::Error;
use ort::session::Session;
use crate::model::Model;

pub enum ModelLoader {
    DefaultModelLoader,
    CustomModelLoader(Box<dyn ModelLoaderTrait>),
}
pub trait ModelLoaderTrait {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>>;
}

#[derive(Default)]
struct DefaultModelLoader;
impl ModelLoaderTrait for DefaultModelLoader {
    fn load(&self, model: Model) -> Result<Session, Box<dyn Error>> {
        todo!()
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