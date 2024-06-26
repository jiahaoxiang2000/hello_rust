use std::{env, process};

use hello_rust_xjh::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    print!("Searching for {} in {}\n", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn iter() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let a = v1.iter().map(|x| x + 1).filter(|x| x > &1).map(|x| x + 1).collect::<Vec<i32>>();
        println!("{:?}", a);
    }
}
