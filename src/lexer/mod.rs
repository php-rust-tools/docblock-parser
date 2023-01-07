use lazy_static::lazy_static;
use regex::bytes::Regex;

use self::{error::LexerError, token::Token};

mod error;
mod span;
mod token;

lazy_static! {
    
}

pub fn tokenize<B: ?Sized + AsRef<[u8]>>(input: &B) -> Result<Vec<Token>, LexerError> {
    
}