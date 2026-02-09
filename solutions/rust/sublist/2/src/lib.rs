#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(a: &[i32], b: &[i32]) -> Comparison {
    use std::cmp::Ordering;

    // 1. 先比较长度
    match a.len().cmp(&b.len()) {
        // 长度相等：直接比较内容是否一致
        Ordering::Equal if a == b => Comparison::Equal,

        // a 比 b 短：检查 a 是否是 b 的子列表
        Ordering::Less if a.is_empty() || b.windows(a.len()).any(|w| w == a) => Comparison::Sublist,

        // a 比 b 长：检查 b 是否是 a 的子列表
        Ordering::Greater if b.is_empty() || a.windows(b.len()).any(|w| w == b) => Comparison::Superlist,

        // 其他所有情况（包括匹配失败）都是 Unequal
        _ => Comparison::Unequal,
    }
}