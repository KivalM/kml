use lexer::lex;
use nodes::Node;
use parser::parse;

pub mod conversion;
pub mod lexer;
pub mod nodes;
pub mod parser;

pub struct KMLDocument {
    document: Node,
}

impl KMLDocument {
    pub fn new(input: &str) -> Result<KMLDocument, String> {
        let document = parse(lex(input))?;
        Ok(KMLDocument { document })
    }

    pub fn to_html(&self) -> String {
        conversion::html::to_html(&self.document)
    }
}
