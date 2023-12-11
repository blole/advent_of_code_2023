use std::error::Error;
use std::io;
use tokenizery::{Tokenizer, Tokenizable, Line};

fn part1<'a, I, E>(
    tokenizer: &'a mut Tokenizer<'a, I, E>,
) -> u32
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizer
        .read_iter::<Line>()
        .map(|result| {
            let line: String = result.unwrap().value;
            let a: char = line.chars().nth(line.find(|c: char| c.is_ascii_digit()).unwrap()).unwrap();
            let b: char = line.chars().nth(line.rfind(|c: char| c.is_ascii_digit()).unwrap()).unwrap();
            a.to_digit(10).unwrap() * 10 + b.to_digit(10).unwrap()
        })
        .sum()
}

fn main() {
    let stdin = io::stdin().lock();
    let mut tokenizer: Tokenizer<_, _> = Tokenizer::new(stdin);

    let part1 = part1(&mut tokenizer);

    println!("{}", part1);
}



#[cfg(test)]
mod tests_day01 {
    use indoc::indoc;
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn p1_0() {
        let mut tokenizer = Tokenizer::from(TEST_INPUT);
        assert_eq!(142, part1(&mut tokenizer));
    }
}
