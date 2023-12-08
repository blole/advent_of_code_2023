use std::error::Error;
use std::io;
use std::marker::PhantomData;
use crate::from_tokenizer::{FromTokenizer, Tokenized};
use crate::tokenizable::Tokenizable;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct Tokenizer<'a, I, E>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizable: I,
    pub(crate) buffer: String,
    phantom: PhantomData<&'a I>,
}

impl<'a, I, E> Tokenizer<'a, I, E>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    pub fn new(
        tokenizable: I,
    ) -> Self {
        Self {
            tokenizable,
            buffer: String::new(),
            phantom: PhantomData
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

    pub fn peek<R: FromTokenizer<'a, I, E, R>>(
        &mut self,
    ) -> Result<Option<R>, E> {
        let mut lookahead = TokenizerLookahead::new(self);
        let Tokenized { value, consumed: _ } = R::peek_from_tokenizer(&mut lookahead)?;
        return Ok(value);
    }

    pub fn read<R: FromTokenizer<'a, I, E, R>> (
        &mut self,
    ) -> Result<Option<R>, E> {
        let mut lookahead = TokenizerLookahead::new(self);
        let Tokenized { value, consumed } = R::peek_from_tokenizer(&mut lookahead)?;
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
    fn temp_peek_until_unicode() {
        let mut tokenizer = Tokenizer::from("a\nb4\u{fe0f}\u{20e3}c4d");
        let mut lookahead = TokenizerLookahead::new(&mut tokenizer);
        assert_eq!("a\nb", lookahead.temp_peek_until("4\u{fe0f}\u{20e3}").unwrap());
        assert_eq!("", lookahead.temp_peek_until("4").unwrap());
        assert_eq!("4\u{fe0f}\u{20e3}", lookahead.temp_peek_until("c").unwrap());
        assert_eq!("c4d", lookahead.temp_peek_until("x").unwrap());
    }

    #[test]
    fn peek_can_peek_line() {
        let mut tokenizer = Tokenizer::from("a\nb\nc");
        let line = tokenizer.peek::<Line>().unwrap().unwrap().value;
        assert_eq!("a\n", line);
    }

    #[test]
    fn read_can_read_char() {
        // 4\u{fe0f}\u{20e3} is "keycap digit four" or :four:
        let mut tokenizer = Tokenizer::from("a\n4\u{fe0f}\u{20e3}c4d");
        assert_eq!('a', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('\n', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('4', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('\u{fe0f}', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('\u{20e3}', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('c', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('4', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!('d', tokenizer.read::<char>().unwrap().unwrap());
        assert_eq!(None, tokenizer.read::<char>().unwrap());
    }
}
