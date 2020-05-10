pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut num = n;
    let mut i = 2;
    while num > 1 {
        if num % i == 0 {
            num /= i;
            factors.push(i);
        } else {
            i += 1;
        }
    }
    factors
}
