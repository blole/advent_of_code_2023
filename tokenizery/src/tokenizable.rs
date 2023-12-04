use std::error::Error;
use std::io;
use std::io::{BufRead, StdinLock};

pub trait Tokenizable<'a> {
    type Err: Error;
    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err>;
}

impl<'a> Tokenizable<'a> for StdinLock<'a> {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err> {
        return self.read_line(buf);
    }
}

impl Tokenizable<'_> for String {
    type Err = io::Error;

    fn tok_read_line(&mut self, buf: &mut String) -> Result<usize, Self::Err> {
        if let Some(newline_index) = self.find('\n') {
            buf.extend(self.drain(..=newline_index));
            Ok(newline_index)
        } else {
            let length = self.len();
            buf.extend(self.drain(..));
            Ok(length)
        }
    }
}
