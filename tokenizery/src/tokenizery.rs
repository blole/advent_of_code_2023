use std::error::Error;
use std::io;
use std::io::{BufRead, StdinLock};
use std::marker::PhantomData;

pub struct Tokenizer<'a, T, E>
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
    pub fn new(
        tokenizable: T,
    ) -> Self {
        Self {
            tokenizable,
            buffer: String::new(),
            phantom: PhantomData
        }
    }

    pub fn read_line(
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

    fn read_until(
        &mut self,
        c: &str,
    ) -> Result<String, E> {
        if let Some(c_index) = self.buffer.find(c) {
            let s = self.buffer.drain(..c_index).collect();
            Ok(s)
        } else {
            loop {
                let read_bytes = self.tokenizable.tok_read_line(&mut self.buffer)?;
                if read_bytes == 0 {
                    let s = self.buffer.drain(..).collect();
                    return Ok(s);
                }
                let offset = self.buffer.len() - read_bytes;
                if let Some(c_index) = self.buffer[offset..].find(c) {
                    let s = self.buffer.drain(..offset + c_index).collect();
                    return Ok(s);
                }
            }
        }
    }

    fn peek_line(
        &mut self,
        offset: usize,
    ) -> Result<&str, E> {
        if let Some(newline_index) = self.buffer[offset..].find('\n') {
            return Ok(&self.buffer[offset..=offset+newline_index])
        }
        self.tokenizable.tok_read_line(&mut self.buffer)?;

        return if let Some(newline_index) = self.buffer[offset..].find('\n') {
            Ok(&self.buffer[offset..=offset+newline_index])
        } else {
            Ok(&self.buffer[offset..])
        }
    }

    pub fn peek<R, F: FromTokenizer<'a, T, E, R>>(
        &'a mut self,
    ) -> Result<Option<R>, E> {
        let mut peeker = TokenizerPeeker::new(self);
        let result = F::peek_from_tokenizer(&mut peeker);
        return result;
    }
}

struct TokenizerPeeker<'a, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizer: &'a mut Tokenizer<'a, T, E>,
    offset: usize,
}

impl<'a, T, E> TokenizerPeeker<'a, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn new(
        tokenizer: &'a mut Tokenizer<'a, T, E>,
    ) -> Self {
        Self {
            tokenizer,
            offset: 0,
        }
    }

    fn temp_peek_line(
        &mut self,
    ) -> Result<&str, E> {
        let line = self.tokenizer.peek_line(self.offset)?;
        self.offset += line.len();
        return Ok(line);
    }

    fn temp_peek_until(
        &'a mut self,
        string: &str,
    ) -> Result<&str, E> {
        let start_offset = self.offset;
        loop {
            let line = self.tokenizer.peek_line(self.offset)?;
            if line.is_empty() {
                return Ok("");
            }
            if let Some(c_index) = line.find(string) {
                self.offset += c_index;
                return Ok(&self.tokenizer.buffer[start_offset..=self.offset]);
            }
            self.offset += line.len();
        }
    }

    fn temp_peek_char(
        &mut self,
    ) -> Result<Option<char>, E> {
        let line = self.tokenizer.peek_line(self.offset)?;
        return if line.len() == 0 {
            Ok(None)
        } else {
            let char = line.chars().next().unwrap();
            self.offset += char.len_utf8();
            Ok(Some(char))
        }
    }
}

struct Line {
    line: String,
}

trait FromTokenizer<'a, T, E, R>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn peek_from_tokenizer(
        peeker: &mut TokenizerPeeker<'a, T, E>,
    ) -> Result<Option<R>, E>;
}

impl<'a, T> FromTokenizer<'a, T, io::Error, Line> for Line
    where
        T: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer(
        peeker: &mut TokenizerPeeker<'a, T, io::Error>,
    ) -> Result<Option<Line>, io::Error> {
        return peeker
            .temp_peek_line()
            .map(|line|
                Some(Line { line: line.to_string() })
            );
    }
}

impl<'a, T> FromTokenizer<'a, T, io::Error, char> for char
    where
        T: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer(
        peeker: &mut TokenizerPeeker<'a, T, io::Error>,
    ) -> Result<Option<char>, io::Error> {
        return peeker.temp_peek_char()
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
    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err>;
}

impl<'a> Tokenizable<'a> for StdinLock<'a> {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err> {
        return self.read_line(buf);
    }
}

impl Tokenizable<'_> for String {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err> {
        if let Some(newline_index) = self.find('\n') {
            buf.extend(self.drain(..=newline_index));
            Ok(newline_index)
        } else {
            let length = self.len();
            buf.extend(self.drain(..));
            Ok(length)
        }
    }
}





#[cfg(test)]
mod tests_day04 {
    use std::io::stdin;
    use super::*;

    #[test]
    fn read_line_simple_cases() {
        let mut tokenizer = Tokenizer::from("a\nb\nc");
        assert_eq!("a\n", tokenizer.read_line().unwrap());
        assert_eq!("b\n", tokenizer.read_line().unwrap());
        assert_eq!("c", tokenizer.read_line().unwrap());
    }

    #[test]
    fn peek_line_simple_cases() {
        let mut tokenizer = Tokenizer::from("a\nb\nc");
        assert_eq!("a\n", tokenizer.peek_line(0).unwrap());
        assert_eq!("a\n", tokenizer.read_line().unwrap());
        assert_eq!("b\n", tokenizer.peek_line(0).unwrap());
        assert_eq!("b\n", tokenizer.peek_line(0).unwrap());
        assert_eq!("b\n", tokenizer.read_line().unwrap());
        assert_eq!("c", tokenizer.peek_line(0).unwrap());
        assert_eq!("c", tokenizer.peek_line(0).unwrap());
        assert_eq!("c", tokenizer.read_line().unwrap());
    }

    #[test]
    fn read_until_simple_cases() {
        let mut tokenizer = Tokenizer::from("abc\ndef");
        assert_eq!("a", tokenizer.read_until("b").unwrap());
        assert_eq!("", tokenizer.read_until("b").unwrap());
        assert_eq!("bc", tokenizer.read_until("\n").unwrap());
        assert_eq!("", tokenizer.read_until("\n").unwrap());
        assert_eq!("\ndef", tokenizer.read_until("x").unwrap());
        assert_eq!("", tokenizer.read_until("x").unwrap());
    }

    #[test]
    fn read_until_unicode() {
        // 4\u{fe0f}\u{20e3} is "keycap digit four" or :four:
        let mut tokenizer = Tokenizer::from("a\nb4\u{fe0f}\u{20e3}c4d");
        assert_eq!("a\nb", tokenizer.read_until("4\u{fe0f}\u{20e3}").unwrap());
        assert_eq!("", tokenizer.read_until("4").unwrap());
        assert_eq!("4\u{fe0f}\u{20e3}", tokenizer.read_until("c").unwrap());
        assert_eq!("c4d", tokenizer.read_until("x").unwrap());
    }
}
