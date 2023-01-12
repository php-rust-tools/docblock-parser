use crate::{bytestring::ByteString, tag::Tag};

#[derive(Debug)]
pub enum Node {
    Text(ByteString),
    Tag(Tag),
}
