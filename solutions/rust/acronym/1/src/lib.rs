pub fn abbreviate(phrase: &str) -> String {
    let mut expanded = String::new();
    let mut prev_char: Option<char> = None;
    for c in phrase.chars() {
        if let Some(prev) = prev_char
            && c.is_uppercase()
            && prev.is_alphabetic()
            && prev.is_lowercase()
        {
            expanded.push(' ');
        }

        expanded.push(c);
        prev_char = Some(c);
    }
    let words: Vec<&str> = expanded.split([' ', '-', '_']).filter(|s| !s.is_empty()).collect();
    words
        .iter()
        .filter_map(|s| s.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .collect()
}
