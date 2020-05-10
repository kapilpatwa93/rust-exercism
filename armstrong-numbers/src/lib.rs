pub fn is_armstrong_number(num: u32) -> bool {
    num.to_string()
        .chars()
        .fold(0, |acc, x| {
            acc + x
                .to_digit(10)
                .unwrap_or(0)
                .pow(num.to_string().len() as u32)
        })
        .eq(&num)
}
