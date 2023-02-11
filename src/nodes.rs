#[derive(Debug)]
pub enum Node {
    Document(Vec<Node>),
    Title(String),
    Date(String),
    Description(String),
    Content(Vec<ContentNode>),
}

#[derive(Debug)]
pub enum ContentNode {
    // paragraph defines a segment of formatted text
    Paragraph(Vec<ContentNode>),
    // link defines a link to another page
    Link(String, Vec<ContentNode>),
    // image defines an image
    Image(String, String),
    // text defines a segment of text
    Text(String),
}
