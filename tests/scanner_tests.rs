use wsh::{scanner::Scanner, scanner::Token};

fn tokenize(input: &str) -> Vec<Token> {
    Scanner::new(input).scan_tokens()
}

#[test]
fn test_scanner() {
    let tokens = tokenize("echo \"hello world\" | wc");
    assert_eq!(
        tokens,
        vec![
            Token::Literal("echo".to_owned()),
            Token::Literal("\"hello world\"".to_owned()),
            Token::Pipe,
            Token::Literal("wc".to_owned()),
            Token::Eof
        ]
    );

    let tokens = tokenize("ls -la");
    assert_eq!(
        tokens,
        vec![
            Token::Literal("ls".to_owned()),
            Token::Literal("-la".to_owned()),
            Token::Eof
        ]
    );

    let tokens = tokenize("echo \" this is | a $ test  \" | wc | wc");
    assert_eq!(
        tokens,
        vec![
            Token::Literal("echo".to_owned()),
            Token::Literal("\" this is | a $ test  \"".to_owned()),
            Token::Pipe,
            Token::Literal("wc".to_owned()),
            Token::Pipe,
            Token::Literal("wc".to_owned()),
            Token::Eof
        ]
    );
}

#[test]
#[should_panic]
fn test_scanner_panic() {
    let tokens = tokenize("ls -la *");
    assert_eq!(
        tokens,
        vec![
            Token::Literal("ls".to_owned()),
            Token::Literal("-la".to_owned()),
            Token::Eof
        ]
    );
}
