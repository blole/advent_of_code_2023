use std::io;
use std::io::StdinLock;
use tokenizery::Tokenizer;

fn main() {
    let stdin = io::stdin().lock();
    let mut tokenizer: Tokenizer<StdinLock, io::Error> = Tokenizer::new(stdin);
}
