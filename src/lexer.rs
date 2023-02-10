#[derive(Debug)]
pub enum Token {
    Title,
    Date,
    Description,
    Content,
    Text(String),
    Indent,
    Dedent,
    Paragraph,
    Link(String),
    Image(String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut indents: Vec<u8> = Vec::new();
    let lines = input.lines();

    for line in lines {
        // skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // check the indentation level of the line
        let indent_level = line.chars().take_while(|c| c.is_whitespace()).count();

        // check if its a top level line
        if indent_level == 0 {
            // we need to dedent all the previous indentation levels
            // and clear the stack
            for _ in 0..indents.len() {
                tokens.push(Token::Dedent);
            }
            indents.clear();
            // we are at the top level so we can just tokenize the line
            tokens.push(tokenize_line(line));
        } else
        // check if the indentation level is greater than the last one
        if indent_level > *indents.last().unwrap_or(&0) as usize {
            // so we are starting a new block with a greater indentation level
            tokens.push(Token::Indent);
            // we need to tokenize the line and add it to the tokens
            tokens.push(tokenize_line(line.trim()));
            // we add the new indentation level to the stack
            indents.push(indent_level as u8);
        } else
        // check if the indentation level is less than the last one
        // this means we are ending a block
        if indent_level < *indents.last().unwrap_or(&0) as usize {
            let mut dedent_count = 0;

            // check how many dedent tokens we need to add
            for indent in indents.iter().rev() {
                if indent_level >= *indent as usize {
                    break;
                }
                dedent_count += 1;
            }

            // add the dedent tokens
            for _ in 0..dedent_count {
                tokens.push(Token::Dedent);
            }

            // remove the dedented indentation levels from the stack
            indents.truncate(indents.len() - dedent_count);

            // we need to check if the indentation level is greater than the last one
            tokens.push(tokenize_line(line.trim()));
        } else {
            // we are at the same indentation level as the last line
            // The case where this happens would be for multiline text or empty blocks
            tokens.push(tokenize_line(line.trim()));
        }
    }

    tokens
}

pub fn tokenize_line(token: &str) -> Token {
    match token {
        "title:" => Token::Title,
        "date:" => Token::Date,
        "description:" => Token::Description,
        "content:" => Token::Content,
        "p:" => Token::Paragraph,
        _ if token.starts_with("link:") => Token::Link(token[5..].trim().to_string()),
        _ if token.starts_with("image:") => Token::Image(token[6..].trim().to_string()),
        txt => Token::Text(txt.trim().to_string()),
    }
}

pub fn print_tokens(tokens: Vec<Token>) {
    let mut indent_str = String::new();

    for token in tokens {
        match token {
            Token::Indent => indent_str.push_str("    "),
            Token::Dedent => {
                indent_str.pop();
                indent_str.pop();
                indent_str.pop();
                indent_str.pop();
            }
            _ => println!("{}{:?}", indent_str, token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = include_str!("../examples/hello.kml");
        let tokens = lex(input);
        print_tokens(tokens);
    }
}
