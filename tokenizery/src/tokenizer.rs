use std::error::Error;
use std::io;
use std::marker::PhantomData;
use crate::from_tokenizer::{FromTokenizer, Tokenized};
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

    pub fn peek<I: FromTokenizer<'a, T, E, I, R>, R>(
        &mut self,
    ) -> Result<R, E> {
        let mut lookahead = TokenizerLookahead::new(self);
        let Tokenized { value, consumed: _ } = I::peek_from_tokenizer(&mut lookahead)?;
        return Ok(value);
    }

    pub fn read<I: FromTokenizer<'a, T, E, I, R>, R>(
        &mut self,
    ) -> Result<R, E> {
        let mut lookahead = TokenizerLookahead::new(self);
        let Tokenized { value, consumed } = I::peek_from_tokenizer(&mut lookahead)?;
        self.buffer.drain(..consumed);
        return Ok(value);
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
mod test_tokenizer {
    use crate::from_tokenizer::Line;
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

    #[test]
    fn peek_can_peek_line_struct() {
        let mut tokenizer = Tokenizer::from("a\nb\nc");
        let line = tokenizer.peek::<Line, String>().unwrap();
        assert_eq!("a\n", line);
    }

    #[test]
    fn read_can_read_char() {
        let mut tokenizer = Tokenizer::from("a\n4\u{fe0f}\u{20e3}c4d");
        assert_eq!('a', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('\n', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('4', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('\u{fe0f}', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('\u{20e3}', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('c', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('4', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!('d', tokenizer.read::<char, Option<char>>().unwrap().unwrap());
        assert_eq!(None, tokenizer.read::<char, Option<char>>().unwrap());
    }
}
