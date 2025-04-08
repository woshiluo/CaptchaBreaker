use std::error::Error;
use ort::session::Session;
use crate::environment::CaptchaEnvironment;
use crate::model::Model;

#[derive(Debug)]
pub enum CaptchaBreaker {
    ChineseClick0,
}
pub trait CaptchaBreakerBuilder {
    type InnerType<'a>: CaptchaBreakerTrait<'a>;
    fn build(self, captcha_environment : &mut CaptchaEnvironment) -> Result<Self::InnerType<'_>, Box<dyn Error>>;
}

impl CaptchaBreakerBuilder for CaptchaBreaker {
    type InnerType<'a> = ChineseClick0<'a>;
    fn build(self, captcha_environment : &mut CaptchaEnvironment) -> Result<Self::InnerType<'_>, Box<dyn Error>> {
        match self {
            CaptchaBreaker::ChineseClick0 => {
                ChineseClick0::build(captcha_environment)
            }
        }
    }
}

pub(crate) trait CaptchaBreakerTrait<'a> {
    fn build(captcha_environment: &'a mut CaptchaEnvironment) -> Result<Self, Box<dyn Error>>
    where Self: Sized;
}


#[derive(Debug)]
pub struct ChineseClick0<'a> {
    yolo11n: &'a Session,
    siamese: &'a Session,
}

impl<'a> CaptchaBreakerTrait<'a> for ChineseClick0<'a> {
    fn build(captcha_environment: &'a mut CaptchaEnvironment) -> Result<ChineseClick0, Box<dyn Error>> {
        let session = captcha_environment.load_models(vec![Model::Yolo11n, Model::Siamese])?;
        Ok(ChineseClick0 {
            yolo11n: session[0],
            siamese: session[1],
        })
    }
}

impl ChineseClick0<'_> {
    pub fn run(&self) -> i8 {
        1
    }
}