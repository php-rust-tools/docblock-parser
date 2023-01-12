use std::{fs::read_dir, path::PathBuf};

#[test]
fn fixtures() -> std::io::Result<()> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tests = manifest.join("tests/fixtures");

    let mut entries = read_dir(tests)?
        .flatten()
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    for entry in entries {
        let doc_filename = entry.join("doc.txt");
        let ast_filename = entry.join("ast.txt");

        if !doc_filename.exists() {
            continue;
        }

        let expected_ast = std::fs::read_to_string(&ast_filename)?;

        let doc = std::fs::read(&doc_filename)?;
        let ast = docblock_parser::parse(&doc);

        assert_eq!(format!("{:#?}", ast), expected_ast);
    }

    Ok(())
}
