# KML - the (o)K(ay) Markup Language

KML is a markup language for defining blog posts. It is designed to be easy to read and write, and to be easy to parse. It is also designed to be easy to convert to other formats, such as HTML for the web, or Markdown for other blogs.

It is a work in progress, and is not yet ready for use. It is currently being developed for use in my own blog, [https://blog.kivalm.com](https://blog.kivalm.com). And will be used in the future for other projects, if I can get it to a usable state.

## Syntax

KML is a simple markup language with a syntax similar to YAML. With its primary purpose being to define blog posts. But realistically, it can be used for any kind of document. 

Blocks are defined by indentation. The first line of a block is the block type, followed by a colon. The rest of the lines are the content of the block. The content of the block is indented by whitespace. The content of the block can be any text, or another block.

### Block Types

#### Title

The title of the document. This is a required block. It is defined by the `title:` block type. The content of the block is the title of the document.

#### Date

The date of the document. This is the second required block. It is defined by the `date:` block type. Currently this is parsed as a string, but in the future it will be parsed as a date. The content of the block is the date that the document was written.

#### Description

The description of the document. It is defined by the `description:` block type. The content of the block is the description of the document. This is used for the meta description of the document.

#### Content

The content of the document. It is defined by the `content:` block type. The content of the block is the content of the document. This is the main part of the document. It will contain all the text, images and links etc, that make up the document.

### Content Blocks

#### Paragraph

A paragraph of text. It is defined by the `p:` block type. The content of the block is the text of the paragraph.

#### Link

A link to another document. It is defined by the `link:url` block type. The content of the block is the text of the link. The link is defined by the block type, followed by a colon, followed by the URL of the link. It works the same way as the anchor tag in HTML.

#### Image

An image. It is defined by the `image:url` block type. The content of the block is the caption of the image. The image is defined by the block type, followed by a colon, followed by the URL of the image.





## Example
```
title:
    Hello World!

date:
    10 February 2023 @ 12PM

description:
    Welcome to my blog!

content:
    p:
        Hi. Welcome to my blog! I hope you enjoy it!
    p:
        This is a link to a Rick Roll.
        A text link is defined like this

        link:https://www.youtube.com/watch?v=dQw4w9WgXcQ
            Click here to be redirected to a Rick Roll!
        
        This is just some plain text below the link.

    p:
        This is an image with a caption.

        image:https://i.ytimg.com/vi/dQw4w9WgXcQ/maxresdefault.jpg
            This is an image of a Rick Roll.

        This is just some plain text below the image.
```