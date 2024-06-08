fn main() {
    // slice
    let mut s = String::from("hello");
    let s1 = &s[0..=1];
    println!(" s1 {}", s1);
    s.push_str( "aaa");
    
}
