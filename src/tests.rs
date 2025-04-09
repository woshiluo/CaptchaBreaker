use crate::captcha::ChineseClick0;
use crate::environment::CaptchaEnvironment;
use std::fs;
use std::io::Write;
use std::time::Instant;

#[test]
fn test1() {
    let environment = CaptchaEnvironment::default(); // 移除 Arc
    let cb: ChineseClick0 = environment.load_captcha_breaker().unwrap();
    let start_time = Instant::now();
    for _ in 0..100 {
        let res = cb.run(&image::open("images/0.jpg").unwrap());
    }
    let duration = start_time.elapsed();
    println!("Time elapsed: {:?}", duration);
}

#[test]
fn test2() {
    let bytes = reqwest::blocking::get("https://www.modelscope.cn/models/Amorter/CaptchaBreakerModels/resolve/master/yolov11n_captcha.onnx").unwrap().bytes().unwrap();
    fs::File::create("models/yolo.onnx")
        .unwrap()
        .write_all(&bytes)
        .unwrap();
}
