use std::{iter::Peekable, panic, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Single-character tokens.
    Dollar,
    LeftParen,
    RightParen,
    Pipe,
    Equal,

    // Literals.
    Word(String),
    String(String),
    Variable(String),

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

        while let Some(c) = self.chars.peek() {
            let token = match c {
                ' ' | '\n' | '\t' => {
                    self.chars.next();
                    continue;
                }
                '(' => self.advance_with(Token::LeftParen),
                ')' => self.advance_with(Token::RightParen),
                '|' => self.advance_with(Token::Pipe),
                '=' => self.advance_with(Token::Equal),
                '$' => {
                    self.chars.next();
                    let mut token = Token::Dollar;
                    if let Some(n) = self.chars.peek() {
                        if n.is_alphanumeric() {
                            token = Token::Variable(self.scan_word())
                        }
                    }
                    token
                }
                '"' => self.scan_string(),
                _ => Token::Word(self.scan_word()),
            };

            tokens.push(token);
        }

        tokens.push(Token::Eof);

        tokens
    }

    fn advance_with(&mut self, token: Token) -> Token {
        self.chars.next();
        token
    }

    fn scan_word(&mut self) -> String {
        let mut word = String::new();
        word.push(self.chars.next().unwrap());

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || *c == '_' {
                word.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        word
    }

    fn scan_string(&mut self) -> Token {
        let mut string = String::new();
        self.chars.next();

        for c in &mut self.chars {
            match c {
                '"' => return Token::String(string),
                _ => string.push(c),
            };
        }
        panic!("string not closed")
    }
}
