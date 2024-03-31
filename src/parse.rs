use tree_sitter::{Language, Parser, Tree};

pub fn parse_text(text: &str) -> Tree {
    let language = tree_sitter_norg::language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    parser.parse(text, None).unwrap()
}
