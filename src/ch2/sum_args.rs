fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut sum = 0;
    for i in 1..args.len() {
        sum += args[i].parse::<i32>().unwrap();
    }
    println!("Sum: {}", sum);
}
