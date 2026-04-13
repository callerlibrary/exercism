pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    let sum: u64 = num.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .map(|digit| digit.pow(len))
        .sum();
    sum == num as u64
}
