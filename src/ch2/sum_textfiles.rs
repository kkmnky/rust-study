use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total: f64 = 0.0;

    for (i, fname) in args.enumerate() {
        if i == 0 {
            continue;
        }
        let contents = fs::read_to_string(fname).unwrap();
        let lines = contents.split("\n");
        for line in lines {
            let num: f64 = match line.parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            total += num;
        }
    }

    println!("Total: {}", total);
}
