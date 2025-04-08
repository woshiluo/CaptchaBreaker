use crate::captcha::{CaptchaBreaker, ChineseClick0};
use crate::environment::CaptchaEnvironment;

#[test]
fn test1() {
    let mut environment = CaptchaEnvironment::default();
    let cb = environment.load_captcha_breaker(CaptchaBreaker::ChineseClick0).unwrap();
    println!("{}", cb.run())
}