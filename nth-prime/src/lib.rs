pub fn nth(num: u32) -> u32 {
    let mut count = 0;
    let mut n:u32 = 3;
    if num == 0 {
        return 2;
    }
    loop {
        if is_prime(n) {
            count += 1;
        }
        if count == num {
            break;
        }
        n += 1;
    }
    return n;
}

fn is_prime(n:u32) -> bool {
    let mut is_prime = true;
    for i in 2..(n as f32).sqrt() as u32 + 1 {
        if n % i == 0 {
            is_prime = false;
            break;
        }
    }
    is_prime
}
