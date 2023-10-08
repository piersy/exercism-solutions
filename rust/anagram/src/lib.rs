use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // This will find anagrams by inserting the word characters into a haset and then checking that
    // they are matched by the possible anagrams.

    let mut m = HashMap::new();
    let mut result: HashSet<&'a str> = HashSet::new();

    let l_word = word.to_lowercase();
    // Add lowercased word chars and count to hashmap.
    for c in l_word.chars() {
        let count = m.entry(c).or_insert(0);
        *count += 1;
    }
    for a in possible_anagrams {
        let l_a = a.to_lowercase();
        if l_a == l_word {
            continue;
        }
        let mut mc = m.clone();
        if is_anagram(&mut mc, &l_a) {
            result.insert(a);
        }
    }
    result
}

fn is_anagram(char_map: &mut HashMap<char, i32>, word: &str) -> bool {
    for c in word.chars() {
        let o = char_map.get_mut(&c);
        match o {
            None => return false,
            Some(v) => {
                *v -= 1;
                if *v < 0 {
                    return false;
                }
            }
        }
    }
    for x in char_map.values() {
        if *x > 0 {
            return false;
        }
    }
    true
}
