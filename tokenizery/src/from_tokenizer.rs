use std::error::Error;
use std::io;

use crate::tokenizable::Tokenizable;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct Tokenized<R> {
    pub value: Option<R>,
    pub consumed: usize,
}

pub trait FromTokenizer<'a, T, E, R>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,

{
    fn peek_from_tokenizer<'b>(
        peeker: &'b mut TokenizerLookahead<'a, 'b, T, E>,
    ) -> Result<Tokenized<R>, E>;
}



pub struct Line {
    pub value: String,
}

impl<'a, T> FromTokenizer<'a, T, io::Error, Line> for Line
    where
        T: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &'b mut TokenizerLookahead<'a, 'b, T, io::Error>,
    ) -> Result<Tokenized<Line>, io::Error> {
        return lookahead
            .temp_peek_line()
            .map(|line|
                Tokenized { value: Some(Line { value: line.to_string() }), consumed: line.len() }
            );
    }
}



impl<'a, T> FromTokenizer<'a, T, io::Error, char> for char
    where
        T: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &'b mut TokenizerLookahead<'a, 'b, T, io::Error>,
    ) -> Result<Tokenized<char>, io::Error> {
        lookahead
            .temp_peek_char()?
            .map_or(Ok(Tokenized { value: None, consumed: 0 }), |c| Ok(Tokenized { value: Some(c), consumed: c.len_utf8() }))
    }
}
