use super::constants;
use rand::Rng;

use super::state;
use std::collections::HashMap;

pub fn generate_new_answer() -> String {
    let mut rng = rand::thread_rng();

    let len = constants::WORDLIST.len();
    let index = rng.gen_range(0, len);
    constants::WORDLIST[index].to_string().to_uppercase()
}

pub fn is_valid_word(word: &str) -> bool {
    for s in constants::WORDLIST.iter() {
        if s.to_uppercase() == word {
            return true;
        }
    }
    false
}

pub fn generate_new_alphabets_status() -> state::AlphabetsStatus {
    let mut hm = HashMap::new();

    for ch in b'A'..=b'Z'{
        hm.insert(ch as char, state::AlphaStatus::Unknown);
    }
    hm
}