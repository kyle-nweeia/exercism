use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let increment_char_cnt = |mut char_cts: HashMap<char, u32>, c| {
        char_cts.entry(c).and_modify(|e| *e += 1).or_insert(0);
        char_cts
    };
    let target = word.to_lowercase();
    let target_char_cts = target.chars().fold(HashMap::new(), increment_char_cnt);

    possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut anagrams, word| {
            let candidate = word.to_lowercase();
            let candidate_char_cts = candidate.chars().fold(HashMap::new(), increment_char_cnt);

            if candidate != target && candidate_char_cts == target_char_cts {
                anagrams.insert(word);
            }

            anagrams
        })
}
