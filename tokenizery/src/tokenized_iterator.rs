use std::error::Error;
use std::marker::PhantomData;
use crate::{Tokenizable, Tokenizer};
use crate::from_tokenizer::FromTokenizer;
use crate::tokenizer_lookahead::TokenizerLookahead;

pub struct TokenizedReadingIterator<'a, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    tokenizer: &'a mut Tokenizer<'a, I, E>,
    phantom: PhantomData<R>,
}

impl<'a, I, E, R> TokenizedReadingIterator<'a, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
{
    pub(crate) fn new(
        tokenizer: &'a mut Tokenizer<'a, I, E>,
    ) -> Self {
        Self {
            tokenizer,
            phantom: PhantomData,
        }
    }
}

impl<'a, I, E, R> Iterator for TokenizedReadingIterator<'a, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        R: FromTokenizer<'a, I, E, R>,
{
    type Item = Result<R, E>;

    fn next(&mut self) -> Option<Result<R, E>> {
        return self.tokenizer.read::<R>().transpose();
    }
}

pub struct TokenizedPeekingIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    lookahead: TokenizerLookahead<'a, 'b, I, E>,
    phantom: PhantomData<R>,
}

impl<'a, 'b, I, E, R> TokenizedPeekingIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        'a: 'b,
{
    pub(crate) fn new(
        tokenizer: &'b mut Tokenizer<'a, I, E>,
    ) -> Self {
        Self {
            lookahead: TokenizerLookahead::new(tokenizer),
            phantom: PhantomData
        }
    }
}

impl<'a, 'b, I, E, R> Iterator for TokenizedPeekingIterator<'a, 'b, I, E, R>
    where
        I: Tokenizable<'a, Err=E>,
        E: Error,
        R: FromTokenizer<'a, I, E, R>,
        'a: 'b,
{
    type Item = Result<R, E>;

    fn next(&mut self) -> Option<Result<R, E>> {
        return R::peek_from_tokenizer(&mut self.lookahead)
            .map(|tokenized| tokenized.value)
            .transpose();
    }
}
