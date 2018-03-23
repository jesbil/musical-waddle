extern crate rand;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};

pub struct MarkovChain {
    map: HashMap<String, Vec<String>>,
    start_words: Vec<String>,
    end_words: Vec<String>
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            map: HashMap::new(),
            start_words: Vec::new(),
            end_words: Vec::new()
        }
    }

    pub fn add_sentence(&mut self, sentence: &str) {
        let words = sentence.split(" ").collect::<Vec<&str>>();
        let word_count = words.len();

        for n in 0..word_count {
            if n + 1 < word_count {
                if (n == 0) {
                    self.start_words.push(words[n].to_string());
                }

                if words[n].ends_with(".") {
                    self.end_words.push(words[n].to_string());
                    self.start_words.push(words[n+1].to_string());
                }

                if words[n].ends_with("!") {
                    self.end_words.push(words[n].to_string());
                    self.start_words.push(words[n+1].to_string());
                }

                if words[n].ends_with("?") {
                    self.end_words.push(words[n].to_string());
                    self.start_words.push(words[n+1].to_string());

                }

                let key = &words[n];
                let value = &words[n + 1];

                self.insert(key.to_string(), value.to_string())
            }
        }
    }

    fn insert(&mut self, key: String, value: String) {
        if self.map.contains_key(&key) {
            let current_value = self.map.get_mut(&key).unwrap();
            current_value.push(value);
        } else {
            self.map.insert(key, vec!(value));
        }
    }

    pub fn generate_sentence(self) -> String {
        let mut rng = thread_rng();

        let mut key = rng.choose(&self.start_words).expect("could not get random value").to_string();
        let mut sentence = key.clone();

        loop {
            match self.map.get(&key) {
                Some(values) => {
                    let value = rng.choose(values).expect("could not get value");
                    sentence = format!("{} {}", sentence, value);

                    if self.end_words.contains(&value) {
                        break
                    }

                    key = value.to_string()
                }
                None => break
            }
        }

        sentence
    }
}

fn next_key(key: &str, value: &str) -> String {
    let last_word = key.split(" ").last().expect("could not get last word");
    format!("{} {}", last_word, value)
}

#[cfg(test)]
pub mod tests {
    use super::MarkovChain;

    #[test]
    fn new() {
        MarkovChain::new();
    }

    #[test]
    fn add_words() {
        let mut markov = MarkovChain::new();
        markov.add_sentence("Hello there! What are you doing here? I really don't like that you are here.");
        let sentence = markov.generate_sentence();
        println!("{:?}", sentence);
    }

}