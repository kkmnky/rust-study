use rand::seq::SliceRandom;

fn main() {
    let mut nums = [0; 75];
    for i in 0..75 {
        nums[i] = i + 1;
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for i in 0..5 {
        for j in 0..5 {
            let index = i * 5 + j;
            if index == 12 {
                print!("  *,");
            } else {
                print!("{:3},", nums[index]);
            }
        }
        println!();
    }
}
