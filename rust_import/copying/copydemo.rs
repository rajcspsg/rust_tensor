fn cop(a: i32, b: i32) {
    println!("a is {} b is {}", a, b);
    println!("{}", a + b);
}

fn main() {
    let a: i32 = 32;
    let b: i32 = 45;
    cop(a, b);
    println!("we have a:{}, b:{}", a, b);
}