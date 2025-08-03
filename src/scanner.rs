use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Pipe,
    Greater,
    Less,
    Literal(String),
    Eof,
}

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.next() {
            let token = match c {
                ' ' | '\n' | '\t' => continue,
                '|' => Token::Pipe,
                '>' => Token::Greater,
                '<' => Token::Less,
                x => {
                    if !self.is_valid_literal_start(&x) {
                        return Err(format!("unexpected token: {x}"));
                    }
                    Token::Literal(self.scan_literal(x))
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);

        Ok(tokens)
    }

    fn scan_literal(&mut self, start: char) -> String {
        if start == '"' {
            return self.scan_string();
        }

        let mut literal = String::new();
        literal.push(start);

        while let Some(&c) = self.chars.peek() {
            if self.is_valid_literal_char(&c) {
                literal.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        literal
    }

    fn scan_string(&mut self) -> String {
        let mut string = String::new();
        for c in &mut self.chars {
            if c == '"' {
                self.chars.next();
                break;
            }
            string.push(c);
        }

        string
    }

    fn is_valid_literal_char(&self, start: &char) -> bool {
        start.is_alphanumeric() || "_-=./:{}\\*;".contains(*start)
    }

    fn is_valid_literal_start(&self, start: &char) -> bool {
        start.is_alphanumeric() || "_-=.\"/:{}\\*;".contains(*start)
    }
}
