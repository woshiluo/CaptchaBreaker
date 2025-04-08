use std::error::Error;
use ort::session::Session;
use crate::environment::CaptchaEnvironment;
use crate::model::Model;

#[derive(Debug)]
pub enum CaptchaBreaker {
    ChineseClick0,
}
impl CaptchaBreaker {
    pub(crate) fn build(self, captcha_environment : &mut CaptchaEnvironment) -> Result<impl CaptchaBreakerTrait, Box<dyn Error>> {
        match self {
            CaptchaBreaker::ChineseClick0 => {
                ChineseClick0::build(captcha_environment)
            }
        }
    }
}

pub(crate) trait CaptchaBreakerTrait {
    fn build(captcha_environment: &mut CaptchaEnvironment) -> Result<impl CaptchaBreakerTrait, Box<dyn Error>> ;
}


#[derive(Debug)]
pub struct ChineseClick0<'a> {
    yolo11n: &'a Session,
    siamese: &'a Session,
}

impl CaptchaBreakerTrait for ChineseClick0<'_> {
    fn build(captcha_environment: &mut CaptchaEnvironment) -> Result<ChineseClick0, Box<dyn Error>> {
        let session = captcha_environment.load_models(vec![Model::Yolo11n, Model::Siamese])?;
        Ok(ChineseClick0 {
            yolo11n: session[0],
            siamese: session[1],
        })
    }

}