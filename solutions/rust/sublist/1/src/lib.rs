#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let len1 = first_list.len();
    let len2 = second_list.len();

    if len1 == 0 && 0 == len2 {
        return Comparison::Equal;
    }

    let (short_list, long_list, key) = if len1 <= len2 {
        (first_list, second_list, 1)
    } else {
        (second_list, first_list, 2)
    };

    let is_sublist = if short_list.is_empty() {
        true
    } else {
        long_list
            .windows(short_list.len())
            .any(|window| window == short_list)
    };

    if is_sublist == false {
        return Comparison::Unequal;
    };

    if key == 1 && len1 == len2 {
        Comparison::Equal
    } else if key == 1 {
        Comparison::Sublist
    } else {
        Comparison::Superlist
    }
}
