pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut num = n;
    while num != 1 {
        if num.is_multiple_of(2) {
            num /= 2;
        } else {
            num = num.checked_mul(3)?.checked_add(1)?;
        }
        steps += 1;
    }

    Some(steps)
}
