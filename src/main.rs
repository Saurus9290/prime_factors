fn main() {
    let number = 60; // You can change this number to test different values
    let prime_factors = factors(number);
    println!("Prime factors of {}: {:?}", number, prime_factors);
}

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;

    while n > 1 {
        let x = candidates.next().unwrap();

        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }

    factors
}
