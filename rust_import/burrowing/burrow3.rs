fn burrow1(v: &Vec<i32>) {
    println!("{}", &v[10] + &v[12]);
}

fn burrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[12]);
}

fn main() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    burrow1(&v);
    println!("still own v {} , {}", v[0], v[1]);

    burrow2(&v);
    println!("still own v {} , {}", v[0], v[1]);
}