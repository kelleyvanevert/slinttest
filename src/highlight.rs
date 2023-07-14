use synoptic::{Highlighter, Token};

use crate::HighlightToken;

pub fn highlight(code: &str) -> Vec<HighlightToken> {
    let mut syntax = Highlighter::new();
    // Add keywords
    syntax.join(&["Sample", "Sin"], "keyword").unwrap();
    // syntax.join(&["bool"], "type").unwrap();
    // syntax.join(&["true", "false"], "boolean").unwrap();
    // // Add comment definitions
    // syntax.add(r"(?m)(//.*)$", "comment").unwrap();
    // syntax.add(r"(?ms)/\*.*?\*/", "comment").unwrap();
    // // Add string definition
    // syntax.add("\".*?\"", "string").unwrap();
    // // Add identifier definition
    // syntax.add(r"([a-z_][A-Za-z0-9_]*)\s*\(", "identifier")
    //     .unwrap();
    // // Add macro definition
    // syntax.add(r"([a-z_][A-Za-z0-9_]*!)\s*", "macro").unwrap();

    let tokens = syntax.run(code);

    let mut highlight_tokens: Vec<HighlightToken> = vec![];

    for (y, row) in tokens.iter().enumerate() {
        let mut x: usize = 0;
        let mut is_keyword = false;

        for token in row {
            match token {
                // Handle the start token (start foreground colour)
                Token::Start(kind) => match kind.as_str() {
                    "keyword" => is_keyword = true,
                    _ => is_keyword = false,
                },
                // Handle a text token (print out the contents)
                Token::Text(text) => {
                    if is_keyword {
                        highlight_tokens.push(HighlightToken {
                            text: text.into(),
                            x: x as i32,
                            y: y as i32,
                        });
                    }

                    is_keyword = false;
                    x += text.len()
                }
                // Handle an end token (reset foreground colour)
                Token::End(_) => {}
            }
        }
    }

    // vec![
    //     HighlightToken {
    //         text: "bass".into(),
    //         x: 0,
    //         y: 0,
    //     },
    //     HighlightToken {
    //         text: "Sin".into(),
    //         x: 21,
    //         y: 0,
    //     },
    //     HighlightToken {
    //         text: "Sin".into(),
    //         x: 36,
    //         y: 0,
    //     },
    //     HighlightToken {
    //         text: "Sin".into(),
    //         x: 51,
    //         y: 0,
    //     },
    //     HighlightToken {
    //         text: "bass".into(),
    //         x: 7,
    //         y: 2,
    //     },
    //     HighlightToken {
    //         text: "bass".into(),
    //         x: 15,
    //         y: 2,
    //     },
    // ]

    highlight_tokens
}
