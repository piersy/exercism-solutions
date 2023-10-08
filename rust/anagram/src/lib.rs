use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Going for a different approach now, sorting and comparing
    let w: Vec<char> = word.to_lowercase().chars().collect();
    let mut sorted_w = w.clone();
    sorted_w.sort_unstable();
    possible_anagrams
        .into_iter()
        .fold(HashSet::new(), |mut acc: HashSet<&str>, v: &&str| {
            // If the length doesn't match then there's no possiblity of being an anagram.
            if v.len() != word.len() {
                return acc;
            }
            let mut lowercase: Vec<char> = v.to_lowercase().chars().collect();
            // Don't include the actual  word
            if w == lowercase {
                return acc;
            }
            lowercase.sort_unstable();
            if sorted_w == lowercase {
                acc.insert(*v);
            }
            acc
        })
}

// fn is_anagram(char_map: &mut HashMap<char, i32>, word: &str) -> bool {
//     for c in word.chars() {
//         let o = char_map.get_mut(&c);
//         match o {
//             None => return false,
//             Some(v) => {
//                 *v -= 1;
//                 if *v < 0 {
//                     return false;
//                 }
//             }
//         }
//     }
//     for x in char_map.values() {
//         if *x > 0 {
//             return false;
//         }
//     }
//     true
// }
