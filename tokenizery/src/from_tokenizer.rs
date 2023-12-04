use std::error::Error;
use std::io;
use crate::tokenizable::Tokenizable;
use crate::Tokenizer;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub trait FromTokenizer<'a, T, E, R>
    where
        T: Tokenizable<'a, Err=E>,
        E: Error,
{
    fn peek_from_tokenizer(
        peeker: &mut TokenizerLookahead<'a, T, E>,
    ) -> Result<Option<R>, E>;
}



pub struct Line {
    line: String,
}

impl<'a, T> FromTokenizer<'a, T, io::Error, Line> for Line
    where
        T: Tokenizable<'a, Err=io::Error>,
{
    fn peek_from_tokenizer(
        peeker: &mut TokenizerLookahead<'a, T, io::Error>,
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
        peeker: &mut TokenizerLookahead<'a, T, io::Error>,
    ) -> Result<Option<char>, io::Error> {
        return peeker.temp_peek_char()
    }
}
