use wsh::{scanner::Scanner, scanner::Token};

fn tokenize(input: &str) -> Vec<Token> {
    Scanner::new(input).scan_tokens()
}

#[test]
fn test_scanner() {
    let tokens = tokenize("=(-24)$()\" $foo \"foo_bar) )");
    assert_eq!(
        tokens,
        vec![
            Token::Equal,
            Token::LeftParen,
            Token::Word("-24".to_owned()),
            Token::RightParen,
            Token::Dollar,
            Token::LeftParen,
            Token::RightParen,
            Token::String(" $foo ".to_owned()),
            Token::Word("foo_bar".to_owned()),
            Token::RightParen,
            Token::RightParen,
            Token::Eof
        ]
    );
}

#[test]
fn test_scanner_strings() {
    let tokens = tokenize("\"hello world\"");
    assert_eq!(
        tokens,
        vec![Token::String("hello world".to_owned()), Token::Eof]
    );

    let tokens = tokenize("$\"foo \" |   \" bar\")");
    assert_eq!(
        tokens,
        vec![
            Token::Dollar,
            Token::String("foo ".to_owned()),
            Token::Pipe,
            Token::String(" bar".to_owned()),
            Token::RightParen,
            Token::Eof
        ]
    );
}

#[test]
fn test_scanner_words() {
    let tokens = tokenize("this is a 123 word");
    assert_eq!(
        tokens,
        vec![
            Token::Word("this".to_owned()),
            Token::Word("is".to_owned()),
            Token::Word("a".to_owned()),
            Token::Word("123".to_owned()),
            Token::Word("word".to_owned()),
            Token::Eof
        ]
    );

    let tokens = tokenize("-1 $(foo) bar-9\" hello\")|");
    assert_eq!(
        tokens,
        vec![
            Token::Word("-1".to_owned()),
            Token::Dollar,
            Token::LeftParen,
            Token::Word("foo".to_owned()),
            Token::RightParen,
            Token::Word("bar".to_owned()),
            Token::Word("-9".to_owned()),
            Token::String(" hello".to_owned()),
            Token::RightParen,
            Token::Pipe,
            Token::Eof
        ]
    );
}

#[test]
fn test_scanner_variables() {
    let tokens = tokenize("$myvar");
    assert_eq!(
        tokens,
        vec![Token::Variable("myvar".to_owned()), Token::Eof]
    );

    let tokens = tokenize("hello$myvar$|$42");
    assert_eq!(
        tokens,
        vec![
            Token::Word("hello".to_owned()),
            Token::Variable("myvar".to_owned()),
            Token::Dollar,
            Token::Pipe,
            Token::Variable("42".to_owned()),
            Token::Eof
        ]
    );
}

#[test]
fn test_empty_input() {
    let tokens = tokenize("");
    assert_eq!(tokens, vec![Token::Eof]);
}

#[test]
#[should_panic(expected = "string not closed")]
fn test_unclosed_string_panics() {
    tokenize("\"hello");
}

#[test]
fn test_mixed_tokens() {
    let tokens = tokenize("echo $VAR | grep \"pattern\" = value");
    assert_eq!(
        tokens,
        vec![
            Token::Word("echo".to_string()),
            Token::Variable("VAR".to_string()),
            Token::Pipe,
            Token::Word("grep".to_string()),
            Token::String("pattern".to_string()),
            Token::Equal,
            Token::Word("value".to_string()),
            Token::Eof
        ]
    );
}
