use crate::{bytestring::ByteString, doc::Doc, node::Node, tag::Tag, r#type::Type, ident_start, ident};

pub fn parse<B: ?Sized + AsRef<[u8]>>(input: &B) -> Doc {
    let input = input.as_ref().to_vec();

    if !is_doc(&input) {
        return Doc::default();
    }

    let number_of_lines = input.iter().filter(|b| **b == b'\n').count();
    let lines = if number_of_lines <= 0 {
        vec![input]
    } else {
        input.split(|b| *b == b'\n').map(|b| b.to_vec()).collect()
    };

    let mut nodes = Vec::new();

    for mut line in lines.into_iter() {
        tidy_up_line(&mut line);

        if line.is_empty() {
            continue;
        }

        match line {
            _ if starts_with_any_of(&line, &[b"@param", b"@phpstan-param", b"@psalm-param"]) => {
                nodes.push(parse_param_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@var", b"@phpstan-var", b"@psalm-var"]) => {
                nodes.push(parse_var_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@return", b"@phpstan-return", b"@psalm-return"]) => {
                nodes.push(parse_return_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@throws", b"@phpstan-throws"]) => {
                nodes.push(parse_throws_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@mixin"]) => {
                nodes.push(parse_mixin_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@deprecated"]) => {
                nodes.push(parse_deprecated_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@param", b"@phpstan-param", b"@psalm-param"]) => {
                nodes.push(parse_param_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@property", b"@property-read", b"@property-write", b"@phpstan-property", b"@phpstan-property-read", b"@phpstan-property-write", b"@psalm-property", b"@psalm-property-read", b"@psalm-property-write"]) => {
                nodes.push(parse_property_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@method", b"@phpstan-method", b"@psalm-method"]) => {
                nodes.push(parse_method_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@template", b"@phpstan-template", b"@psalm-template", b"@template-covariant", b"@phpstan-template-covariant", b"@psalm-template-covariant", b"@template-contravariant", b"@phpstan-template-contravariant", b"@psalm-template-contravariant"]) => {
                nodes.push(parse_template_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@extends", b"@phpstan-extends", b"@template-extends"]) => {
                nodes.push(parse_extends_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@implements", b"@phpstan-implements", b"@template-implements"]) => {
                nodes.push(parse_implements_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@use", b"@phpstan-use", b"@template-use"]) => {
                nodes.push(parse_use_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@phpstan-type", b"@psalm-type"]) => {
                nodes.push(parse_type_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@phpstan-import-type", b"@psalm-import-type"]) => {
                nodes.push(parse_import_type_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@phpstan-assert", b"@phpstan-assert-if-true", b"@phpstan-assert-if-false", b"@psalm-assert", b"@psalm-assert-if-true", b"@psalm-assert-if-false"]) => {
                nodes.push(parse_assert_tag(line))
            },
            _ if starts_with_any_of(&line, &[b"@phpstan-this-out", b"@phpstan-self-out", b"@psalm-this-out", b"@psalm-self-out"]) => {
                nodes.push(parse_self_out_tag(line))
            }
            _ if starts_with_any_of(&line, &[b"@param-out", b"@phpstan-param-out", b"@psalm-param-out"]) => {
                nodes.push(parse_param_out_tag(line))
            }
            // TODO: Handle unknown tags here. Pattern should check if
            //      line starts with `@`, followed by any ASCII alpha character.
            //      If that's not found, fallback to regular text node collection.
            _ => todo!("this should produce a text node once other tags are handled."),
        }
    }

    Doc { nodes }
}

fn parse_param_out_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_self_out_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_assert_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_import_type_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_type_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_use_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_implements_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_extends_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_template_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_method_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_property_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_deprecated_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_mixin_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_throws_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_return_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_var_tag(line: Vec<u8>) -> Node {
    todo!()
}

fn parse_param_tag(mut line: Vec<u8>) -> Node {
    let name = skip_over_any_of(&mut line, &[b"@param", b"@phpstan-param", b"@psalm-param"]);
    skip_whitespace(&mut line);

    let r#type = parse_optional_type(&mut line);
    skip_whitespace(&mut line);

    // FIXME: Also need to check for variadic / reference parameters here.
    let variable = read_required_variable(&mut line);
    skip_whitespace(&mut line);

    let description = read_optional_description(&mut line);

    Node::Tag(Tag::Param { name, r#type, variable, description })
}

fn parse_optional_type(line: &mut Vec<u8>) -> Option<Type> {
    if ! can_see_type_string(line) {
        return None;
    }
    
    if let Some(identifier) = try_read_identifier(line) {
        // FIXME: We can be smarter here and convert native types into dedicated enum members.
        let mut _type = Type::Identifier(identifier);
        Some(_type)
    } else {
        todo!()
    }
}

fn try_read_identifier(line: &mut Vec<u8>) -> Option<ByteString> {
    if ! matches!(line[0], ident_start!()) {
        return None;
    }

    let mut bytes = Vec::new();

    bytes.push(line[0]);
    line.drain(..1);

    loop {
        if line.is_empty() {
            break;
        }

        if matches!(line[0], ident!()) {
            bytes.push(line[0]);
            line.drain(..1);
        } else {
            break;
        }
    }
    
    Some(bytes.into())
}

// FIXME: Description for some tags could span multiple lines.
//        Will need to add support for this eventually, it could
//        probably be done with a second pass over the AST to squash
//        text nodes down with their previous sibling.
fn read_optional_description(line: &mut Vec<u8>) -> Option<ByteString> {
    let mut bytes = Vec::new();
    
    if line.is_empty() {
        return None;
    }

    while !line.is_empty() {
        match line[0] {
            b'\n' => break,
            _ => {
                bytes.push(line[0]);
                line.drain(..1);
            },
        }
    }

    if bytes.is_empty() { None } else { Some(bytes.into()) }
}

fn skip_over_any_of(line: &mut Vec<u8>, patterns: &[&[u8]]) -> ByteString {
    for pattern in patterns {
        if line.starts_with(pattern) {
            return ByteString::from(line.drain(..pattern.len()).collect::<Vec<u8>>());
        }
    }

    unreachable!()
}

fn can_see_type_string(line: &[u8]) -> bool {
    match line[..2] {
        [b'$', b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'\x80'..=b'\xff'] => false,
        _ => true,
    }
}

fn read_required_variable(line: &mut Vec<u8>) -> ByteString {
    let mut bytes = Vec::new();

    match line[..2] {
        [b'$', ident_start!()] => {
            bytes.push(line[0]);
            bytes.push(line[1]);
            line.drain(..2);
        },
        _ => panic!("bad variable. FIXME: Be more tolerant and return an error instead.")
    };

    loop {
        if line.is_empty() {
            break;
        }

        match line[0] {
            ident!() => {
                bytes.push(line[0]);
                line.drain(..1);
            },
            _ => break,
        }
    }

    bytes.into()
}

fn skip_whitespace(line: &mut Vec<u8>) {
    while line.starts_with(b" ") {
        line.drain(..1);
    }
}

fn tidy_up_line(line: &mut Vec<u8>) {
    if line.starts_with(b"/*") {
        line.drain(..2);
    }

    if line.ends_with(b"*/") {
        line.drain(line.len() - 2..);
    }

    if line.starts_with(b" ") {
        line.drain(..1);
    }

    if line.starts_with(b"*") {
        line.drain(..1);
    }

    if line.starts_with(b" ") {
        line.drain(..1);
    }

    if line.ends_with(b" ") {
        line.drain(line.len() - 1..);
    }
}

fn is_doc(input: &[u8]) -> bool {
    input.starts_with(b"/**")
}

fn starts_with_any_of(subject: &[u8], patterns: &[&[u8]]) -> bool {
    for pattern in patterns {
        if subject.starts_with(pattern) {
            return true;
        }
    }

    false
}
