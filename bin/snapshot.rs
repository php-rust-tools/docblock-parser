use std::{fs::read_dir, path::PathBuf};

use docblock_parser::parse;

fn main() -> std::io::Result<()> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut entries = read_dir(manifest.join("tests/fixtures"))?
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

        if ast_filename.exists() {
            std::fs::remove_file(&ast_filename)?;
        }

        let doc = std::fs::read(&doc_filename)?;
        let ast = parse(&doc);

        std::fs::write(&ast_filename, format!("{:#?}", ast))?;
    }

    Ok(())
}
