use std::error::Error;
use std::io;
use std::io::{BufRead, StdinLock};
use std::marker::PhantomData;
use std::str::FromStr;

struct Tokenizer<'a, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizable: T,
    buffer: String,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, E> Tokenizer<'a, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn new(
        tokenizable: T,
    ) -> Self {
        Self {
            tokenizable,
            buffer: String::new(),
            phantom: PhantomData
        }
    }

    fn read_line(
        &mut self,
    ) -> Result<String, E> {
        if let Some(newline_index) = self.buffer.find('\n') {
            let line = self.buffer.drain(..=newline_index).collect();
            Ok(line)
        } else {
            self.tokenizable.tok_read_line(&mut self.buffer)?;
            Ok(std::mem::take(&mut self.buffer))
        }
    }
}

impl<T> From<T> for Tokenizer<'_, String, io::Error>
    where
        T: ToString,
{
    fn from(value: T) -> Self {
        Self::new(value.to_string())
    }
}

trait Tokenizable<'a> {
    type Err: Error;
    fn tok_read_line(&mut self, buf: &mut String) -> Result<(), Self::Err>;
}

impl<'a> Tokenizable<'a> for StdinLock<'a> {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<(), Self::Err> {
        return self.read_line(buf).map(|_| ());
    }
}

impl Tokenizable<'_> for String {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<(), Self::Err> {
        if let Some(newline_index) = self.find('\n') {
            buf.extend(self.drain(..=newline_index));
            Ok(())
        } else {
            buf.extend(self.drain(..));
            Ok(())
        }
    }
}

// #[derive(Debug)]
fn main() {
    let stdin = io::stdin().lock();
    let mut tokenizer: Tokenizer<StdinLock, io::Error> = Tokenizer::new(stdin);
    let line1 = tokenizer.read_line().unwrap();
    let line2 = tokenizer.read_line().unwrap();

    println!("Hello, world! {} {}", line1, line2);
}

#[cfg(test)]
mod tests_day04 {
    use std::io::stdin;
    use super::*;

    const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

         22 13 17 11  0
          8  2 23  4 24
         21  9 14 16  7
          6 10  3 18  5
          1 12 20 15 19

          3 15  0  2 22
          9 18 13 17  5
         19  8  7 25 23
         20 11 10 24  4
         14 21 16 12  6

         14 21 17 24  4
         10 16 15  9 19
         18  8 23 26 20
         22 11 13  6  5
          2  0 12  3  7
         ";

    #[test]
    fn read_line_simple_cases() {
        let mut tokenizer = Tokenizer::from("a\nb\nc");
        assert_eq!("a\n", tokenizer.read_line().unwrap());
        assert_eq!("b\n", tokenizer.read_line().unwrap());
        assert_eq!("c", tokenizer.read_line().unwrap());
    }

    #[test]
    fn p2_0() {
        //let diagnostic_report = DIAGNOSTIC_REPORT.map(|line| i32::from_str_radix(&line, 2).unwrap());
        //assert_eq!(230, part2(&diagnostic_report.to_vec(), 5));
    }
}
