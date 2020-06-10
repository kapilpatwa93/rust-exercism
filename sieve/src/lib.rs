pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![];
    let mut arr: Vec<u64> = (0..=upper_bound).collect();
    for x in 2..=upper_bound {
        if arr[x as usize] != 0 {
            primes.push(x);
            if x * x > upper_bound {
                break;
            }
            for n in arr.iter_mut().step_by(x as usize) {
                if *n != x {
                    *n = 0;
                }
            }
        }
    }
    arr.iter()
        .filter(|x| **x != 0 && **x != 1)
        .map(|&x| x)
        .collect()
}
