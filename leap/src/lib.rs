pub fn is_leap_year(year: u64) -> bool {
    let mut res = false;
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                res = true;
            } else {
                res = false;
            }
        }
        res = true;
    }
    res
}
