fn main() {
    let mut ss: Vec<String> = Vec::new();
    ss.push("hello".to_string());

    println!("{}", ss); // This will print `hello, world!`
}

fn one_list(ss: &Vec<String>) -> String {
    ss[0]
}
