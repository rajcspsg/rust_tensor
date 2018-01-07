fn take(v: Vec<i32>) {
    println!("we took v: {}", v[10] + v[100]);
}

fn main() {
    let mut v = Vec::new();
    for i in 1..1000 {
        v.push(i);
    }
    println!("V[10] is {}", v[10]);
    println!("V[100] is {}", v[100]);
    take(v);
    //println!("V[0] is {}", v[0]);
    println!("Finished");
}