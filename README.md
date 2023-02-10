# KML - the (o)K(ay) Markup Language

KML is a markup language for defining blog posts. It is designed to be easy to read and write, and to be easy to parse. It is also designed to be easy to convert to other formats, such as HTML for the web, or Markdown for other blogs.

It is a work in progress, and is not yet ready for use. It is currently being developed for use in my own blog, [https://blog.kivalm.com](https://blog.kivalm.com). And will be used in the future for other projects, if I can get it to a usable state.

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