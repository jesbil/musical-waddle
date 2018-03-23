pub mod markovchain;
use markovchain::MarkovChain;
pub mod file_parser;
fn main() {
    let contents = file_parser::parse_file("markov/resources/nice.txt");
    let splitted = contents.split("\n");
    let sentences = splitted.collect::<Vec<&str>>();
    let mut markov = MarkovChain::new();

    for sentence in sentences {
        markov.parse(sentence);
    }

    println!("{:?}",markov.generate_sentence());

}
