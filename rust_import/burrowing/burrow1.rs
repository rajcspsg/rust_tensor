//import std::vec::Vec;

fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn burrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]);
}

fn burrow2(v: &Vec<i32>) {
    println!("{}", &v[10] + &v[12]);
}

fn main() {
    let mut v =  Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    v = re(v);
    println!("still own v {} , {}", v[0], v[1]);

    burrow1(&v);
    println!("still own v {} , {}", v[0], v[1]);

    burrow2(&v);
    println!("still own v {} , {}", v[0], v[1]);
}