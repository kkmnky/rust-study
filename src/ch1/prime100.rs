fn is_prime(n: usize) -> bool {
    if n < 2 { return false; }
    let mut i = 2;
    while i*i <= n {
        if n % i == 0 { return false; }
        i += 1;
    }
    return true;
}

fn get_primes(primes: &mut[usize; 100]) {
    let mut i = 2;
    let mut count = 0;

    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut primes = [0; 100];
    get_primes(&mut primes);
    println!("{:?}", primes);
}