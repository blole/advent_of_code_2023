use std::io;
use std::io::StdinLock;
use tokenizery::Tokenizer;

fn main() {
    let stdin = io::stdin().lock();
    let mut tokenizer: Tokenizer<StdinLock, io::Error> = Tokenizer::new(stdin);
    let line1 = tokenizer.read_line().unwrap();
    let line2 = tokenizer.read_line().unwrap();

    println!("Hello, world! {} {}", line1, line2);
}
