use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Divison,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operation(Operator),
    Number(usize),
}

struct Calculator {
    expr: String,
}

trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for Calculator {
    fn tokenize(&self) -> Vec<Token> {
        let mut it = self.expr.chars().peekable();
        let mut tokens: Vec<Token> = vec![];

        loop {
            consume_while(&mut it, char::is_whitespace);

            match it.peek() {
                Some(&ch) => {
                    let token = match ch {
                        '0'...'9' => {
                            let num: String = consume_while(&mut it, char::is_numeric)
                                .into_iter()
                                .collect();
                            Token::Number(num.parse().unwrap())
                        }
                        '+' => {
                            it.next().unwrap();
                            Token::Operation(Operator::Addition)
                        }
                        '-' => {
                            it.next().unwrap();
                            Token::Operation(Operator::Subtraction)
                        }
                        _ => panic!("unsupported character"),
                    };
                    tokens.push(token);
                }
                None => break,
            }
        }

        tokens
    }
}

fn consume_while<F: Fn(char) -> bool>(it: &mut Peekable<Chars>, test: F) -> Vec<char> {
    let mut chars = vec![];

    while let Some(&ch) = it.peek() {
        if test(ch) {
            let ch = it.next().unwrap();
            chars.push(ch);
        } else {
            break;
        }
    }
    chars
}

#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
    let expect = vec![
        Token::Number(5),
        Token::Operation(Operator::Addition),
        Token::Number(8),
    ];
    let expr = Calculator { expr: String::from("5+8") };
    assert_eq!(expr.tokenize(), expect);
}

#[test]
fn test_sub_add() {
    assert_eq!(2 + 2, 4);
    let expect = vec![
        Token::Number(5),
        Token::Operation(Operator::Subtraction),
        Token::Number(8),
        Token::Operation(Operator::Addition),
        Token::Number(9),
    ];
    let expr = Calculator { expr: String::from("5-8+9") };
    assert_eq!(expr.tokenize(), expect);
}

#[test]
fn test_tokenize_spaces() {
    assert_eq!(2 + 2, 4);
    let expect = vec![
        Token::Number(5),
        Token::Operation(Operator::Addition),
        Token::Number(8),
    ];
    let expr = Calculator { expr: String::from(" 5 +  8 ") };
    assert_eq!(expr.tokenize(), expect);
}
