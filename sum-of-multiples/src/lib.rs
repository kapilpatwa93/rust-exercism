pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = vec![];
    for i in 0..limit {
        if is_multiple(i, factors) {
            multiples.push(i);
        }
    }
    multiples.iter().sum::<u32>()
}
fn is_multiple(num: u32, factors: &[u32]) -> bool {
    let mut is_multiple: bool = false;
    for i in factors {
        if *i != 0 && num % *i == 0 {
            is_multiple = true;
            break;
        }
    }
    is_multiple
}
