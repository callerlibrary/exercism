pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut remainder = n;
    let mut candidate = 2;

    while candidate * candidate <= remainder {
        while remainder.is_multiple_of(candidate) {
            result.push(candidate);
            remainder /= candidate;
        }
        candidate += 1;
    }

    if remainder > 1 {
        result.push(remainder);
    }

    result
}
