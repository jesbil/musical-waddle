// This is a comment, and will be ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
mod markovchain;
use markovchain::MarkovChain;
mod file_parser;
fn main() {
    file_parser::parse_file("markov/resources/nice.txt");
}
