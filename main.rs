use std::ptr::eq;

fn main() {
    // deep clone
    let a = "Hello".to_string();
    let b = &a;
    println!("a = {}, b = {}, a==b {}", &a, &b, eq(&a, b));
    // copy clone
    let a = 1;
    let b = a;
    println!("a = {}, b = {}, a==b {}", &a, &b, eq(&a, &b));
}
