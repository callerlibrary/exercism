pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 { return vec![]; }
    let width = garden[0].len();

    // 1. 初始化计数板（一维化存储）
    let mut counts = vec![0u8; height * width];

    // 2. 遍历：仅当遇到花时，推送更新给邻居
    for r in 0..height {
        for c in 0..width {
            if garden[r].as_bytes()[c] == b'*' {
                // 更新邻居
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

    // 3. 构建结果字符串
    (0..height).map(|r| {
        let mut row_str = String::with_capacity(width);
        for c in 0..width {
            if garden[r].as_bytes()[c] == b'*' {
                row_str.push('*');
            } else {
                let count = counts[r * width + c];
                row_str.push(if count == 0 { ' ' } else { (count + b'0') as char });
            }
        }
        row_str
    }).collect()
}
