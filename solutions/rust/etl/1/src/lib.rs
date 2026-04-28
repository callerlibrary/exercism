use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();
    for (&score, letters) in h {
        for &ch in letters {
            res.insert(ch.to_ascii_lowercase(), score);
        }
    }
    res
}

