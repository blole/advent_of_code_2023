use std::error::Error;
use crate::tokenizable::Tokenizable;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct Tokenized<R> {
    pub value: Option<R>,
    pub consumed: usize,
}

pub trait FromTokenizer<'a, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,

{
    fn peek_from_tokenizer<'b>(
        lookahead: &mut TokenizerLookahead<'a, 'b, I, E>,
    ) -> Result<Tokenized<R>, E>;
}



pub struct Line {
    pub value: String,
}

impl<'a, I, E> FromTokenizer<'a, I, E, Line> for Line
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &mut TokenizerLookahead<'a, 'b, I, E>,
    ) -> Result<Tokenized<Line>, E> {
        return lookahead
            .temp_peek_line()
            .map(|line|
                if line.is_empty() {
                    Tokenized { value: None, consumed: 0 }
                } else {
                    Tokenized { value: Some(Line { value: line.trim_end_matches("\n").to_string() }), consumed: line.len() }
                }
            );
    }
}



impl<'a, I, E> FromTokenizer<'a, I, E, char> for char
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &mut TokenizerLookahead<'a, 'b, I, E>,
    ) -> Result<Tokenized<char>, E> {
        lookahead
            .temp_peek_char()?
            .map_or(Ok(Tokenized { value: None, consumed: 0 }), |c| Ok(Tokenized { value: Some(c), consumed: c.len_utf8() }))
    }
}
