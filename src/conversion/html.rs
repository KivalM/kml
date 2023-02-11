use crate::nodes::{ContentNode, Node};

pub fn to_html(document: &Node) -> String {
    let mut html = String::new();

    fn recurse(node: &Node, html: &mut String) {
        match node {
            Node::Document(nodes) => {
                html.push_str("<div id='document'>");
                for node in nodes {
                    recurse(node, html);
                }
                html.push_str("</div>");
            }
            Node::Title(title) => {
                html.push_str("<h1 id='title'>");
                html.push_str(title);
                html.push_str("</h1>");
            }
            Node::Date(date) => {
                html.push_str("<h2 id='date'>");
                html.push_str(date);
                html.push_str("</h2>");
            }
            Node::Description(description) => {
                html.push_str("<p id='description'>");
                html.push_str(description);
                html.push_str("</p>");
            }
            Node::Content(nodes) => {
                html.push_str("<div id='content'>");
                for node in nodes {
                    recurse_content_node(node, html);
                }
                html.push_str("</div>");
            }
        }
    }

    fn recurse_content_node(node: &ContentNode, html: &mut String) {
        match node {
            ContentNode::Paragraph(p) => {
                html.push_str("<p id='paragraph'>");
                for node in p {
                    recurse_content_node(node, html);
                }
                html.push_str("</p>");
            }
            ContentNode::Link(url, nodes) => {
                html.push_str("<a href='");
                html.push_str(url);
                html.push_str("'>");
                for node in nodes {
                    recurse_content_node(node, html);
                }
                html.push_str("</a>");
            }
            ContentNode::Image(url, caption) => {
                html.push_str("<figure>");
                html.push_str("<img src='");
                html.push_str(url);
                html.push_str("'>");
                html.push_str("<figcaption>");
                html.push_str(caption);
                html.push_str("</figcaption>");
                html.push_str("</figure>");
            }
            ContentNode::Text(text) => {
                html.push_str(text);
            }
        }
    }

    recurse(document, &mut html);
    html
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    #[test]
    fn test_html_conversion() {
        let input = include_str!("../../examples/hello_world/hello_world.kml");
        let document = parse(lex(input)).unwrap();
        let html = to_html(&document);
        println!("HTML: {}", html);
    }
}
