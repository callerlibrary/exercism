pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 { return vec![]; }
    let width = garden[0].len();

    let mut counts = vec![0u8; height * width];

    // 使用 .iter().enumerate() 代替 0..height
    for (r, row_str) in garden.iter().enumerate() {
        // 直接迭代每一行的字节，避免再次索引
        for (c, &byte) in row_str.as_bytes().iter().enumerate() {
            if byte == b'*' {
                // 只有在这里我们才需要计算坐标范围
                for dr in r.saturating_sub(1)..=(r + 1).min(height - 1) {
                    for dc in c.saturating_sub(1)..=(c + 1).min(width - 1) {
                        if dr != r || dc != c {
                            counts[dr * width + dc] += 1;
                        }
                    }
                }
            }
        }
    }

    // 构建结果时也可以使用 enumerate
    garden.iter().enumerate().map(|(r, row_str)| {
        row_str.as_bytes().iter().enumerate().map(|(c, &byte)| {
            if byte == b'*' {
                '*'
            } else {
                match counts[r * width + c] {
                    0 => ' ',
                    n => (n + b'0') as char,
                }
            }
        }).collect()
    }).collect()
}
