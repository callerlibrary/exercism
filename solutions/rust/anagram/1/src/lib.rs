use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let word_lower = word.to_lowercase();
    let word_sorted = to_sorted(&word_lower);

    let possible_anagrams:Vec<&str> = possible_anagrams
        .iter()
        .filter(|&&s| {
            let s_lower = s.to_lowercase();
            if s_lower.len() != word_lower.len() {
                return false;
            }

            if s_lower == word_lower {
                return false;
            }

            word_sorted == to_sorted(&s_lower)
        })
        .copied()
        .collect::<Vec<_>>();

    HashSet::from_iter(possible_anagrams)
}

fn to_sorted(s: &str) -> Vec<char> {
    let mut word_sorted = s.chars().collect::<Vec<_>>();
    word_sorted.sort_unstable();
    word_sorted
}