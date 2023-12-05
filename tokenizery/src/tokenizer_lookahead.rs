use std::error::Error;
use crate::tokenizable::Tokenizable;
use crate::tokenizer::Tokenizer;

pub struct TokenizerLookahead<'a, 'b, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    tokenizer: &'b mut Tokenizer<'a, T, E>,
    pub(crate) offset: usize,
}

impl<'a, 'b, T, E> TokenizerLookahead<'a, 'b, T, E>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    pub(crate) fn new(
        tokenizer: &'b mut Tokenizer<'a, T, E>,
    ) -> Self {
        Self {
            tokenizer,
            offset: 0,
        }
    }

    pub(crate) fn temp_peek_line(
        &mut self,
    ) -> Result<&str, E> {
        let line = self.tokenizer.peek_line(self.offset)?;
        self.offset += line.len();
        return Ok(line);
    }

    pub(crate) fn temp_peek_until(
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

    pub(crate) fn temp_peek_char(
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
