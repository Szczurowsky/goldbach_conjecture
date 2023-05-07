use std::io::{Write};

pub fn check_goldbach(mut n: usize) {
    while can_be_expressed_as_sum_of_primes(n) {
        if n % 10000 == 0 {
            let file_path = std::path::Path::new("last_number.txt");
            let mut file = std::fs::File::create(file_path).unwrap();
            file.write_all(n.to_string().as_bytes()).unwrap();
        }
        n += 2;
    }
    println!("The Goldbach conjecture is false for the even integer: {}", n);
    let success_file = std::path::Path::new("possible'nt.txt");
    let mut file = std::fs::File::create(success_file).unwrap();
    file.write_all(n.to_string().as_bytes()).unwrap();
}

// Determines whether `n` is prime by checking divisibility from 2 to sqrt(n)+1.
// Returns true if `n` is prime, false otherwise.
pub fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let upper_bound = (n as f64).sqrt() as usize + 1;
    for i in 2..upper_bound {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn can_be_expressed_as_sum_of_primes(n: usize) -> bool {
    if n <= 2 || n % 2 != 0 {
        return false;
    }
    for i in 2..n {
        if is_prime(i) && is_prime(n - i) {
            return true;
        }
    }
    false
}