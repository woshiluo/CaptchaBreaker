use crate::captcha::ChineseClick0;
use crate::environment::CaptchaEnvironment;

#[test]
fn test1() {
    let environment = CaptchaEnvironment::default(); // 移除 Arc
    let cb: ChineseClick0 = environment.load_captcha_breaker().unwrap();
    let cb1: ChineseClick0 = environment.load_captcha_breaker().unwrap();

    drop(environment);

    println!("{}  {}", cb.run(), cb1.run());
}