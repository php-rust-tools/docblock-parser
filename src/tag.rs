use crate::{bytestring::ByteString, r#type::Type};

#[derive(Debug)]
pub enum Tag {
    Param {
        name: ByteString,
        r#type: Option<Type>,
        variable: ByteString,
        description: Option<ByteString>,
    },
}
