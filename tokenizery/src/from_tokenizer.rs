use std::error::Error;
use std::io;

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
        peeker: &'b mut TokenizerLookahead<'a, 'b, I, E>,
    ) -> Result<Tokenized<R>, E>;
}



pub struct Line {
    pub value: String,
}

impl<'a, I> FromTokenizer<'a, I, io::Error, Line> for Line
    where
        I: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &'b mut TokenizerLookahead<'a, 'b, I, io::Error>,
    ) -> Result<Tokenized<Line>, io::Error> {
        return lookahead
            .temp_peek_line()
            .map(|line|
                Tokenized { value: Some(Line { value: line.to_string() }), consumed: line.len() }
            );
    }
}



impl<'a, I> FromTokenizer<'a, I, io::Error, char> for char
    where
        I: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer<'b>(
        lookahead: &'b mut TokenizerLookahead<'a, 'b, I, io::Error>,
    ) -> Result<Tokenized<char>, io::Error> {
        lookahead
            .temp_peek_char()?
            .map_or(Ok(Tokenized { value: None, consumed: 0 }), |c| Ok(Tokenized { value: Some(c), consumed: c.len_utf8() }))
    }
}
