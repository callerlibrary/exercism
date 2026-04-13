/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // 1. 清理数据：过滤掉所有空格
    let cleaned: String = code.chars().filter(|c| !c.is_whitespace()).collect();

    // 2. 基础验证：
    // - 长度必须大于 1
    // - 不能包含任何非数字字符
    if cleaned.len() <= 1 || cleaned.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    // 3. 执行 Luhn 算法
    // 我们从末尾（右侧）开始计算，所以使用 .rev() 翻转
    let sum: u32 = cleaned
        .chars()
        .rev() // 从右向左
        .enumerate() // 获取索引以确定哪些位需要翻转
        .map(|(i, c)| {
            let mut n = c.to_digit(10).unwrap(); // 已经检查过合法性，这里用 unwrap 是安全的

            // 每隔一个数字（即偶数索引位，因为索引从 0 开始且已经翻转）
            // 实际上题目说“右边起第二个”，翻转后对应的索引就是 1, 3, 5...
            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            n
        })
        .sum();

    // 4. 最终检查是否能被 10 整除
    sum.is_multiple_of(10)
}
