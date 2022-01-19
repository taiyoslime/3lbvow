use super::constants;
use rand::Rng;

pub fn generate_new_answer() -> &'static str {
    let mut rng = rand::thread_rng();

    let len = constants::WORDLIST.len();
    let index = rng.gen_range(0, len);
    constants::WORDLIST[index]
}

pub fn is_valid_word(word: &str) -> bool {
    // TODO
    for s in constants::WORDLIST.iter() {
        if *s == word {
            return true;
        }
    }
    false
}
