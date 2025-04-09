use std::error::Error;
use std::rc::Rc;
use ort::session::Session;
use crate::environment::CaptchaEnvironment;
use crate::model::Model;
pub trait CaptchaBreaker {
    fn build(captcha_environment: &CaptchaEnvironment) -> Result<Self, Box<dyn Error>>
    where Self: Sized;
}

#[derive(Debug)]
pub struct ChineseClick0 {
    yolo11n: Rc<Session>,
    siamese: Rc<Session>,
}

impl CaptchaBreaker for ChineseClick0 {
    fn build(captcha_environment: &CaptchaEnvironment) -> Result<Self, Box<dyn Error>> {
        let session = captcha_environment.load_models(vec![Model::Yolo11n, Model::Siamese])?;
        Ok(ChineseClick0 {
            yolo11n: session[0].clone(),
            siamese: session[1].clone(),
        })
    }
}

impl ChineseClick0 {
    pub fn run(&self) -> i8 {
        1
    }
}