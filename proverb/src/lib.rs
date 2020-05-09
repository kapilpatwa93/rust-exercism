pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::from("");
    }
    let mut proverb: Vec<String> = vec![];
    for i in list.iter().enumerate() {
        proverb.push(format!(
            "For want of a {} the {} was lost.",
            list[i - 1],
            list[i]
        ))
    }

    proverb.push(format!("And all for the want of a {}.", list[0]));
    proverb.join("\n")
}
