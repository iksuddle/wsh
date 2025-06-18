use wsh::{scanner::Scanner, scanner::Token};

fn tokenize(input: &str) -> Vec<Token> {
    Scanner::new(input).scan_tokens()
}

#[test]
fn test_scanner() {
    let tokens = tokenize("=(-24)() foo_bar)\" this is| a $() test  \"|");
    assert_eq!(
        tokens,
        vec![
            Token::Equal,
            Token::LeftParen,
            Token::Literal("-24".to_owned()),
            Token::RightParen,
            Token::LeftParen,
            Token::RightParen,
            Token::Literal("foo_bar".to_owned()),
            Token::RightParen,
            Token::Literal("\" this is| a $() test  \"".to_owned()),
            Token::Pipe,
            Token::Eof
        ]
    );
}
