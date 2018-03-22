mod markovchain;
use markovchain::MarkovChain;
mod file_parser;
fn main() {
    file_parser::parse_file("markov/resources/nice.txt");
}
