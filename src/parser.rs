use lalrpop_util::lalrpop_mod;

use crate::{prelude::Document, document::Type};

lalrpop_mod!(internal);

pub fn parse(input: &str) -> Document {
    internal::DocumentParser::new().parse(input).unwrap()
}

pub fn parse_type(input: &str) -> Type {
    internal::TypeParser::new().parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{document::Variable, prelude::Node};

    use super::parse;

    #[test]
    fn simple_param_tag_with_variable() {
        assert_eq!(
            parse("/** @param $hello */").collect(),
            vec![Node::ParamTag {
                tag: "param".into(),
                _type: None,
                variable: Variable {
                    name: "hello".into(),
                    reference: false,
                    variadic: false
                }
            }]
        );

        assert_eq!(
            parse("/** @phpstan-param $hello */").collect(),
            vec![Node::ParamTag {
                tag: "phpstan-param".into(),
                _type: None,
                variable: Variable {
                    name: "hello".into(),
                    reference: false,
                    variadic: false
                }
            }]
        );

        assert_eq!(
            parse("/** @psalm-param $hello */").collect(),
            vec![Node::ParamTag {
                tag: "psalm-param".into(),
                _type: None,
                variable: Variable {
                    name: "hello".into(),
                    reference: false,
                    variadic: false
                }
            }]
        );
    }
}
