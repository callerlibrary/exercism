pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return Vec::new();
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|chunk| std::str::from_utf8(chunk).unwrap().to_string())
        .collect()
}
