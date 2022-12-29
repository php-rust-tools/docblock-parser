use lalrpop_util::lalrpop_mod;

use crate::prelude::Document;

lalrpop_mod!(internal);

pub fn parse(input: &str) -> Document {
    internal::DocumentParser::new().parse(input).unwrap()
}