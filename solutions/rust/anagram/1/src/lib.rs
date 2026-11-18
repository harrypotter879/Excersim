use std::collections::HashSet;
fn check_anagram(word : &str, ana : &str) -> bool{
    if word.len() != ana.len() || word.to_lowercase() == ana.to_lowercase(){
        return false
    }
    
    let mut v1: Vec<char> = word.to_lowercase().chars().collect();
    let mut v2: Vec<char> = ana.to_lowercase().chars().collect();

    v1.sort();
    v2.sort();

    v1 == v2
}


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str>{
    possible_anagrams 
        .iter()
    .filter(|&&x| check_anagram(word, x))
    .copied()
    .collect()
}
