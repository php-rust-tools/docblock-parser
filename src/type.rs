use crate::bytestring::ByteString;

#[derive(Debug)]
pub enum Type {
    Identifier(ByteString),
}