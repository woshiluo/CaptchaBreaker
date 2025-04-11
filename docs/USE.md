### 使用文档

每个验证码类型都是一个CaptchaBreaker，拥有自己的模型，考虑到模型可能被多个CB复用，使用env来管理所有模型的加载和所有权

- 默认使用cpu执行器和网络加载器(从modelscope下载模型到本地models文件夹)
```rust
use crate::environment::CaptchaEnvironment;
let environment = CaptchaEnvironment::default();
```

你需要从env中加载需要的验证码类型
```rust
use crate::captcha::ChineseClick0;
let cb: ChineseClick0 = environment.load_captcha_breaker().unwrap();
```
注意，目前env的模型使用Rc(引用计数器)来管理，所以drop(env)后模型不会被销毁。
后期会加入运行时处理内存释放的相关逻辑。