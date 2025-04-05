use ndarray::{s, Array4};

#[test]
fn test_hello() {
    let a = Array4::<f32>::zeros((1, 3, 386, 386));
    let b = a.slice(s![.., .., .., 0]);
    println!("{:?}", b);
}