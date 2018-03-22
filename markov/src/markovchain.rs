extern crate rand;

use std::collections::HashMap;
use self::rand::{thread_rng, Rng};

pub struct MarkovChain {
    map: HashMap<String, Vec<String>>
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            map: HashMap::new()
        }
    }

    pub fn parse(&mut self, sentence: &str) {
        let words = sentence.split(" ").collect::<Vec<&str>>();
        let word_count = words.len();

        for n in 0..word_count {
            if n + 2 < word_count {
                let key = format!("{} {}", words[n], words[n + 1]);
                let value = &words[n + 2];

                self.insert(key, value.to_string())
            } else {
                break;
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
        let keys = self.map.keys().collect::<Vec<&String>>();

        let mut key = rng.choose(&keys).expect("could not get random value").to_string();
        let mut sentence = key.clone();

        loop {
            match self.map.get(&key) {
                Some(values) => {
                    let value = rng.choose(values).expect("could not get value");
                    sentence = format!("{} {}", sentence, value);

                    key = next_key(&key, value);
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
mod tests {
    use super::MarkovChain;

    #[test]
    fn new() {
        MarkovChain::new();
    }

    #[test]
    fn add_words() {
        let mut markov = MarkovChain::new();
        markov.parse("I REALLY LIKE TO SAY HELLO AND TO SAY THAT THIS IS A NICE WORLD WE HAVE.");
        let sentence = markov.generate_sentence();
        println!("{:?}", sentence);
    }

}