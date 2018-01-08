
In Rust, simple data types (fixed length literals) are stored on stack.
Complex data types likes like vector, String (variable sized) types are stored on heap.
Due to the complexity os heap, it is slower than stack.

In C/C++, programmer have to allocate and deallocate the memory. In higher programming languages like java, gargbage collector runs and dereferences the objects which are no longer need and eligible for free memory.

<h1> Ownership</h1>

In Rust, we've something in between. It is called ownership.

ownership rules - 

1. Each object in the heap has owner [Owner can be variable in the code].
2. Each object can have only one owner at any point of time.
3. When the owner goes out of scope, the value will be dropped. After every closing blocks of code, the objects are checked for drop eligibility.

Transfer Ownership 

```
fn main() {
   let s = String::from("string");
   let y = s;
   //println!("s is {}", s);
}
```

In the above code, accessing `s` after assigning it to y is invalid because the ownership is transferred to y. 
Remember, there can be only one owner for object at a time.

<h1>Burrowing</h1>

To fix the ownership transfer error above, we need to use the concept called 'Burrow'.

```
fn main() {
   let s = String::from("string");
   let y = &s;
   println!("s is {}", s);
}
```

In the above program, at line 3, `&` is used while assign to y. It means y burrowed the reference to s and now we can use y and s repeatedly in subsequent lines.

<h1>Moving</h1>

```
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
```

 In the above program explained as,
 
 1. we have created variable sized vector and it will be stored on heap.
 2. for loop puts the data into the vector.
 3. When we call take the ownership is transferred to take function and it doesn't return anything. as per rule 3 of ownership, it would be dropped at the end of scope that is take function is completed.
 
 Moving is changing the ownership from one function to another function.
 
 <h1>Copying</h1>
 
 ```
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
 
```

The a and b are used in main and cop becuase ownership never gets tansferred for fixed literals. The values are copied.

<h1>Burrow Again</h1>


```
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
```
1. re takes the ownership and returns the ownership of v.
2. burrow1 takes the reference of or burrows v.
3. burrow2 takes the reference of or burrows v.


```
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
```
Here the ownership of v is burrow by the loop and the count function.



