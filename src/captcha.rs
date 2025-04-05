use std::error::Error;
use std::sync::Arc;
use ort::session::Session;
use crate::config::{DefaultLoader, ModelLoader};
use crate::model::{ModelEnum};

enum CaptchaEnum {
    ChineseClick0,
}

struct CaptchaEngine {
    captcha_breaker: Box<dyn CaptchaBreaker>,
}

impl CaptchaEngine {
    fn build() -> CaptchaEngineBuilder {
        CaptchaEngineBuilder::new()
    }
}

struct CaptchaEngineBuilder {
    model_loader: Option<Arc<dyn ModelLoader>>,
    captcha_breaker: Option<CaptchaEnum>,
}
impl CaptchaEngineBuilder {
    fn new() -> Self {
        CaptchaEngineBuilder{
            model_loader: None,
            captcha_breaker: None,
        }
    }
    fn with_model_loader(mut self, model_loader: Arc<dyn ModelLoader>) -> Self {
        self.model_loader = Some(model_loader);
        self
    }
    fn with_captcha_breaker(mut self, breaker: CaptchaEnum) -> Self {
        self.captcha_breaker = Some(breaker);
        self
    }
    fn build(self) -> Result<CaptchaEngine, Box<dyn Error>> {
        let model_loader = self.model_loader.unwrap_or_else(|| Arc::from(DefaultLoader::default()));
        // todo!(错误处理)
        let captcha_breaker = Box::from(match self.captcha_breaker.unwrap() {
            CaptchaEnum::ChineseClick0 => {
                ChineseClick0 {
                    yolo11n_model: model_loader.load(ModelEnum::Yolo11n)?,
                    siamese_model: model_loader.load(ModelEnum::Siamese)?,
                }
            }
        });
        Ok(CaptchaEngine {
            captcha_breaker
        })
    }
}

trait CaptchaBreaker {
    fn run(&self, inputs: dyn Inputs) -> Result<dyn Outputs, Box<dyn Error>>;
}

trait Outputs {}
trait Inputs {}

#[cfg(feature = "chinese_click_0")]
struct ChineseClick0 {
    yolo11n_model: Session,
    siamese_model: Session,
}
impl CaptchaBreaker for ChineseClick0 {}