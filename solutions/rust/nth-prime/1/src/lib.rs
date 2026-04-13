pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }

    let mut count = 1; // 已经找到2和3
    let mut num = 5; // 从5开始检查

    while count < n {
        if is_prime(num) {
            count += 1;
            if count == n {
                return num;
            }
        }
        // 检查6k+1 (num + 2)
        num += 2;
        if is_prime(num) {
            count += 1;
            if count == n {
                return num;
            }
        }
        num += 4; // 跳到下一组6k±1 (从5->11)
    }
    num
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }

    let mut i = 5;
    let limit = n.isqrt();

    while i <= limit {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}
