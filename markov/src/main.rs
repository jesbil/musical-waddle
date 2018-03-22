mod markovchain;
use markovchain::MarkovChain;
mod file_parser;
fn main() {
    let contents = file_parser::parse_file("markov/resources/nice.txt");
    let splitted = contents.split("\n");
    let sentences = splitted.collect::<Vec<&str>>();
    println!("{:?}",sentences);

}
