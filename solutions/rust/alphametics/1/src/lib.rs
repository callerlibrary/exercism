use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input = input.replace(' ', "");
    let (left, right) = input.split_once("==")?;

    let words: Vec<&str> = left.split('+').collect();

    let mut letters_set = HashSet::new();
    let mut non_zero = HashSet::new();

    for w in words.iter().chain(std::iter::once(&right)) {
        let w = w.trim();

        if let Some(c) = w.chars().next()
            && w.len() > 1
        {
            non_zero.insert(c);
        }

        for c in w.chars() {
            if c.is_ascii_alphabetic() {
                letters_set.insert(c);
            }
        }
    }

    if letters_set.len() > 10 {
        return None;
    }

    let letters: Vec<char> = letters_set.into_iter().collect();

    let mut domain: HashMap<char, Vec<u8>> = HashMap::new();

    for &c in &letters {
        let mut v: Vec<u8> = (0..10).collect();

        // 非零约束
        if non_zero.contains(&c) {
            v.retain(|&x| x != 0);
        }

        domain.insert(c, v);
    }

    // 回溯状态
    let mut used = [false; 10];
    let mut assign: HashMap<char, u8> = HashMap::new();

    fn word_to_num(word: &str, map: &HashMap<char, u8>) -> Option<u64> {
        let mut n = 0u64;

        for c in word.chars() {
            let d = *map.get(&c)? as u64;
            n = n * 10 + d;
        }

        Some(n)
    }

    fn check(words: &Vec<&str>, right: &str, map: &HashMap<char, u8>) -> bool {
        let sum: Option<u64> = words
            .iter()
            .map(|w| word_to_num(w, map))
            .collect::<Option<Vec<_>>>()
            .map(|v| v.iter().sum());

        let res = word_to_num(right, map);

        match (sum, res) {
            (Some(s), Some(r)) => s == r,
            _ => false,
        }
    }

    fn dfs(
        i: usize,
        letters: &Vec<char>,
        domain: &HashMap<char, Vec<u8>>,
        used: &mut [bool; 10],
        assign: &mut HashMap<char, u8>,
        words: &Vec<&str>,
        right: &str,
    ) -> bool {
        if i == letters.len() {
            return check(words, right, assign);
        }

        let c = letters[i];

        for &d in &domain[&c] {
            if used[d as usize] {
                continue;
            }

            used[d as usize] = true;
            assign.insert(c, d);

            if dfs(i + 1, letters, domain, used, assign, words, right) {
                return true;
            }

            used[d as usize] = false;
            assign.remove(&c);
        }

        false
    }

    if dfs(0, &letters, &domain, &mut used, &mut assign, &words, right) {
        Some(assign)
    } else {
        None
    }
}
