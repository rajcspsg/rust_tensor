fn main() {

    let v = vec![4,5,6,3,6,7,4,8,6,7,4,4,3,8];

    for &i in &v {
        let r = count(&v, i);
        println!("{} is present in the vecor {} times", i, r);
    }

}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}