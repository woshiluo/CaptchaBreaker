use criterion::{black_box, criterion_group, criterion_main, Criterion};
use captcha_breaker::captcha::ChineseClick0;
use captcha_breaker::environment::CaptchaEnvironment;


pub fn criterion_benchmark(c: &mut Criterion) {
    let environment = CaptchaEnvironment::default();
    let cb : ChineseClick0 = environment.load_captcha_breaker().unwrap();
    let image = image::open("images/0.jpg").unwrap();
    c.bench_function("chinese_click_0", |b| b.iter(||
        {
            cb.run(black_box(&image));
        }
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);