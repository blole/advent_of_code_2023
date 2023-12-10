use std::error::Error;
use std::marker::PhantomData;
use crate::{Tokenizable, Tokenized, Tokenizer};
use crate::from_tokenizer::FromTokenizer;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct TokenizedIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    lookahead: TokenizerLookahead<'a, 'b, I, E>,
    consuming: bool,
    phantom: PhantomData<R>,
}

impl<'a, 'b, I, E, R> TokenizedIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    pub(crate) fn new(
        tokenizer: &'b mut Tokenizer<'a, I, E>,
        consuming: bool,
    ) -> Self {
        Self {
            lookahead: TokenizerLookahead::new(tokenizer),
            consuming,
            phantom: PhantomData
        }
    }
}

impl<'a, 'b, I, E, R> Iterator for TokenizedIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        R: FromTokenizer<'a, I, E, R>,
        'a: 'b,
{
    type Item = Result<R, E>;

    fn next(&mut self) -> Option<Result<R, E>> {
        let peeked = R::peek_from_tokenizer(&mut self.lookahead);
        if let Ok(Tokenized { value, consumed }) = peeked {
            if self.consuming {
                self.lookahead.tokenizer.buffer.drain(..consumed);
                self.lookahead.offset -= consumed;
            }
            return value.map(|v| Ok(v));
        } else {
            return Some(Err(peeked.err().unwrap()));
        }
    }
}
