pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut lines: Vec<String> = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect();

    lines.push(format!("And all for the want of a {}.", list[0]));

    lines.join("\n")
}
