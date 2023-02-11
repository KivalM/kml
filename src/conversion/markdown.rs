use crate::nodes::{ContentNode, Node};

pub fn to_markdown(document: &Node) -> String {
    let mut markdown = String::new();

    fn recurse(node: &Node, markdown: &mut String) {
        match node {
            Node::Document(nodes) => {
                for node in nodes {
                    recurse(node, markdown);
                    markdown.push_str("\n");
                }
            }
            Node::Title(title) => {
                markdown.push_str("# ");
                markdown.push_str(title);
                markdown.push_str(" #");
            }
            Node::Date(date) => {
                markdown.push_str("## ");
                markdown.push_str(date);
                markdown.push_str(" ##");
            }
            Node::Description(description) => {
                markdown.push_str("### ");
                markdown.push_str(description);
                markdown.push_str(" ###");
            }
            Node::Content(nodes) => {
                for node in nodes {
                    recurse_content_node(node, markdown);
                }
            }
        }
    }

    fn recurse_content_node(node: &ContentNode, markdown: &mut String) {
        match node {
            ContentNode::Paragraph(p) => {
                for node in p {
                    recurse_content_node(node, markdown);
                }
                markdown.push_str("\n\n");
            }
            ContentNode::Link(url, nodes) => {
                markdown.push_str("[");
                for node in nodes {
                    recurse_content_node(node, markdown);
                }
                markdown.push_str("](");
                markdown.push_str(url);
                markdown.push_str(")");
            }
            ContentNode::Image(url, caption) => {
                // we can use a markdown table here
                // | ![space-1.jpg](http://www.storywarren.com/wp-content/uploads/2016/09/space-1.jpg) |
                // |:--:|
                // | *Space* |

                markdown.push_str("| ![](");
                markdown.push_str(url);
                markdown.push_str(") |");
                markdown.push_str("\n");
                markdown.push_str("|:--:|");
                markdown.push_str("\n");
                markdown.push_str("| *");
                markdown.push_str(caption);
                markdown.push_str("* |");
                markdown.push_str("\n");
            }
            ContentNode::Text(text) => {
                markdown.push_str(text);
            }
        }
    }

    recurse(document, &mut markdown);
    markdown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;
    use crate::parser::parse;

    #[test]
    fn test_markdown_conversion() {
        let input = include_str!("../../examples/hello_world/hello_world.kml");
        let document = parse(lex(input)).unwrap();
        let html = to_markdown(&document);
        println!("MD: {}", html);
    }
}
