use std::{iter::Peekable, panic, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Pipe,
    Equal,

    // Literals.
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.next() {
            let token = match c {
                ' ' | '\n' | '\t' => {
                    continue;
                }
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '|' => Token::Pipe,
                '=' => Token::Equal,
                x => Token::Literal(self.scan_literal(x)),
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);

        tokens
    }

    fn scan_literal(&mut self, start: char) -> String {
        if start == '"' {
            return self.scan_string(start);
        }

        let mut literal = String::new();
        literal.push(start);

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || *c == '_' || *c == '-' {
                literal.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        literal
    }

    fn scan_string(&mut self, start: char) -> String {
        let mut string = String::new();
        string.push(start);

        while let Some(c) = self.chars.next() {
            string.push(c);
            if c == '"' {
                break;
            }
        }

        string
    }
}
