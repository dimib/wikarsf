// Title: Styles
use std::fs::read_to_string;

pub fn default_style() -> String {
    String::from(
        r#"
        <style>
        body {
            font-family: sans-serif;
            font-size: 16px;
            line-height: 1.5;
            color: #333;
            background-color: #fff;
            margin: 0;
            padding: 0;
        }
        h1, h2, h3, h4, h5, h6 {
            margin: 0;
            padding: 0;
            font-weight: normal;
        }
        h1 {
            font-size: 2em;
        }
        h2 {
            font-size: 1.5em;
        }
        h3 {
            font-size: 1.17em;
        }
        h4 {
            font-size: 1em;
        }
        h5 {
            font-size: 0.83em;
        }
        h6 {
            font-size: 0.67em;
        }
        p {
            margin: 0;
            padding: 0;
        }
        ul {
            margin: 0;
            padding: 0;
        }
        li {
            margin: 0;
            padding: 0;
        }
        blockquote {
            margin: 0;
            padding: 0;
        }
        code {
            margin: 0;
            padding: 0;
        }
        pre {
            margin: 0;
            padding: 0;
        }
        </style>
        "#,
    )
}

pub fn inline_style(path: String) -> Result<String, std::io::Error> {
    let file_content = read_to_string(path)?;
    Ok(file_content)
}