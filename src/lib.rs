use lexer::lex;
use nodes::Node;
use parser::parse;

pub mod conversion;
pub mod lexer;
pub mod nodes;
pub mod parser;

pub struct KML {
    document: Node,
}

impl KML {
    pub fn new(input: &str) -> Result<KML, String> {
        let document = parse(lex(input))?;
        Ok(KML { document })
    }

    pub fn to_html(&self) -> String {
        conversion::html::to_html(&self.document)
    }
}
