## 验证码类型

如果要使用某个验证码类型，你需要开启相应的feature，例如，ChineseClick0对应的feature为chinese_click_0

### ChineseClick0
![ChineseClick0](images/chinese_click_0.jpg)

整个验证码为一张图片，HW为(384, 344)

输入参数：验证码图片

输出参数：一个Vec<(x:f32,y:f32)> 为按顺序需要点选的坐标 (左上角为坐标原点)