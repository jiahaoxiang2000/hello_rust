
fn main() {
    // reference
    let mut s = String::from("hello");
    let  b1 =  &mut s;
    println!("b1: {}", b1);
    
    let  b2 =  &s;
    let  b3 =  &s;
    println!("b2 : {}", b2);
    println!("b3 : {}", b3);

    s.push(char::from(33));
}
