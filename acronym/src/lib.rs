pub fn abbreviate<'a>(phrase: &'a str) -> String {
    let splitters: Vec<&str> = vec!["", ".", "_", "-"];
    // creating vector of closures, for every symbol(pattern) you want to split the word with
    let split_by_closures: Vec<Box<dyn Fn(&'a str) -> Vec<&'a str>>> = splitters
        .iter()
        .skip(1) // skip "" as you dont want to split every characters
        .map(|&pattern| {
            Box::new(move |word: &'a str| -> Vec<&'a str> {
                word.split_terminator(pattern).collect()
            }) as Box<dyn Fn(&'a str) -> Vec<&'a str>>
        })
        .collect();

    let mut iter: Vec<&str> = phrase.split_whitespace().collect();
    // run all the split_by_closures for the given phrase
    split_by_closures.iter().for_each(|split_by| {
        iter = iter.iter().flat_map(|word| split_by(word)).collect();
    });
    iter.iter()
        .flat_map(|w: &&str| {
            let mut last_uppercase_position: usize = 0;
            let mut uppercase_count: usize = 0;
            w.chars().enumerate().for_each(|(index, ch)| {
                if ch.is_ascii_uppercase() {
                    uppercase_count += 1;
                    last_uppercase_position = index;
                }
            });
            if uppercase_count == 2 {
                let ts = w.split_at(last_uppercase_position);
                vec![ts.0, ts.1]
            } else {
                vec![*w]
            }
        })
        .filter(|s: &&str| !splitters.contains(s))
        .fold(String::from(""), |acc: String, str: &str| -> String {
            format!("{}{}",acc,str.chars().next().unwrap().to_ascii_uppercase())
        })
}
