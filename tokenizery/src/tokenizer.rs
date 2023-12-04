use std::error::Error;
use std::io;
use std::io::BufRead;
use std::marker::PhantomData;
use crate::from_tokenizer::FromTokenizer;
use crate::tokenizable::Tokenizable;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct Tokenizer<'a, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizable: T,
    pub(crate) buffer: String,
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

    pub(crate) fn peek_line(
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
        let mut peeker = TokenizerLookahead::new(self);
        let result = F::peek_from_tokenizer(&mut peeker);
        return result;
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




#[cfg(test)]
mod tests_day04 {
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
