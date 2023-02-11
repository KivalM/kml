use crate::{
    lexer::Token,
    nodes::{ContentNode, Node},
};

pub fn parse(tokens: Vec<Token>) -> Result<Node, String> {
    // a reasonable starting point for the parser would be to accumulate
    // all consecutive text tokens since we don't care about newlines.
    let tokens = concatenate_consecutive_text_tokens(tokens);

    // declare a vector to hold all the nodes
    let mut nodes = Vec::new();

    // This function will recurse through the top level nodes and process them recursively
    // It will return the index of the last token it processed
    fn recurse_root_level(mut index: usize, tokens: &Vec<Token>, nodes: &mut Vec<Node>) -> usize {
        // all the top level nodes are either a title, date, description or content
        // and they all follow a similar pattern where its the tag, then an indent, then the text
        let root_token = &tokens[index];

        match root_token {
            Token::Title | Token::Date | Token::Description => {
                // assert that the next token is an indent
                index += 1;
                let indent_token = &tokens[index];
                assert!(
                    matches!(indent_token, Token::Indent),
                    "Expected indent token after title token but got {:?}",
                    indent_token
                );

                // now we take the next token and check if it is a text token
                // add the node to the nodes vec
                index += 1;
                let text_token = &tokens[index];

                match text_token {
                    Token::Text(text) => {
                        let node = match root_token {
                            Token::Title => Node::Title(text.to_string()),
                            Token::Date => Node::Date(text.to_string()),
                            Token::Description => Node::Description(text.to_string()),
                            _ => unreachable!(),
                        };
                        nodes.push(node);
                    }
                    _ => {
                        panic!(
                            "Expected text token after indent token but got {:?}",
                            text_token
                        );
                    }
                }
            }

            Token::Content => {
                // after the content token, we expect an indent token
                index += 1;
                let indent_token = &tokens[index];
                assert!(
                    matches!(indent_token, Token::Indent),
                    "Expected indent token after content token but got {:?}",
                    indent_token
                );

                // now we want to recurse into the content level
                // we create a vector to hold all the content nodes
                let mut content_nodes = Vec::new();

                // in this section, indent levels are important
                // we want to keep track of the current indent level
                index = recurse_content_level(index + 1, 1, tokens, &mut content_nodes);
                nodes.push(Node::Content(content_nodes));
            }

            _ => {
                // we want to ignore all other tokens
            }
        }

        // if we are not at the end of the tokens, we want to recurse
        if index < tokens.len() {
            recurse_root_level(index + 1, tokens, nodes)
        } else {
            0
        }
    }

    // This function will recurse through the content level nodes and process them recursively
    // It will return the index of the last token it processed
    fn recurse_content_level(
        mut index: usize,
        mut indent_level: usize,
        tokens: &Vec<Token>,
        nodes: &mut Vec<ContentNode>,
    ) -> usize {
        if index >= tokens.len() {
            return index;
        }

        match &tokens[index] {
            Token::Text(text) => {
                // if we encounter a text token, we want to push it to the nodes vec
                nodes.push(ContentNode::Text(text.to_string()));
            }
            Token::Indent => {
                // if we encounter an indent token, we want to increment the indent level

                indent_level += 1;
            }
            Token::Dedent => {
                // if we encounter a dedent token, we want to decrement the indent level
                indent_level -= 1;

                // we can end the content-level recursion by checking if the current token brings us back to the root level
                if indent_level == 1 {
                    return index;
                }
            }
            Token::Paragraph => {
                // if we encounter a paragraph token, we want to recurse into the paragraph level
                let mut paragraph_nodes = Vec::new();
                index = recurse_content_level(index + 1, 1, tokens, &mut paragraph_nodes);
                nodes.push(ContentNode::Paragraph(paragraph_nodes));
            }
            Token::Link(url) => {
                // if we encounter a link token, we want to recurse into the link level
                let mut link_nodes = Vec::new();
                index = recurse_content_level(index + 1, 1, tokens, &mut link_nodes);
                nodes.push(ContentNode::Link(url.to_string(), link_nodes));
            }
            Token::Image(url) => {
                // if we encounter a link token, we want to recurse into the link level
                let mut link_nodes = Vec::new();
                index = recurse_content_level(index + 1, 1, tokens, &mut link_nodes);
                if let Some(ContentNode::Text(text)) = link_nodes.first() {
                    nodes.push(ContentNode::Image(url.to_string(), text.to_string()));
                } else {
                    panic!(
                        "Expected a single text node after image token but got {:?}",
                        link_nodes
                    );
                }
            }

            token => {
                panic!("Unexpected token: {:?}", token);
            }
        }

        recurse_content_level(index + 1, indent_level, tokens, nodes)
    }

    recurse_root_level(0, &tokens, &mut nodes);
    Ok(Node::Document(nodes))
}

/// This function will concatenate all consecutive text tokens into a single text token
/// This is useful for the parser, as it will make it easier to parse the text nodes and ignore
/// the whitespace
///
/// TODO: Find a way to do this without allocating a new vector
fn concatenate_consecutive_text_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::new();
    let mut text = String::new();

    for token in tokens {
        match token {
            // we only want to concatenate text tokens with a space
            Token::Text(t) => {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(&t);
            }
            // if we encounter a non-text token,
            // we end the current text token and push it
            _ => {
                // if the text is not empty, we want to push it
                // and reset the text variable
                if !text.is_empty() {
                    new_tokens.push(Token::Text(text));
                    text = String::new();
                }
                new_tokens.push(token);
            }
        }
    }

    // if there is any text left over, we want to push it
    if !text.is_empty() {
        new_tokens.push(Token::Text(text));
    }

    new_tokens
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;

    use super::*;

    #[test]
    fn test() {
        let input = include_str!("../examples/hello_world/hello_world.kml");
        let tokens = lex(input);
        let nodes = parse(tokens).unwrap();
        println!("{:#?}", nodes);
    }
}
