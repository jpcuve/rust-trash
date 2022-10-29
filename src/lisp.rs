use std::{env, io};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
enum Token {
    Nil,
    Symbol(String),
    Number(f64),
    String(String),
    List(Vec<Token>)
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Token::Nil => write!(f, "()"),
            Token::Symbol(symbol) => write!(f, "{}", symbol),
            Token::Number(number) => write!(f, "{}", number),
            Token::String(string) => write!(f, "\"{}\"", string),
            Token::List(v) => {
                write!(f, "(")?;
                let mut space = false;
                for token in v {
                    if space {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", token)?;
                    space = true;
                }
                write!(f, ")")
            },
        }
    }
}

fn parse(token_iterator: &mut core::slice::Iter<&str>) -> Result<Token, &'static str> {
    let token = match token_iterator.next() {
        Some(s) => {
            match *s {
                "(" => {
                    let mut list: Vec<Token> = vec![];
                    loop {
                        let token = parse(token_iterator)?;
                        if let Token::Nil = token {
                            break;
                        } else {
                            list.push(token);
                        }
                    }
                    Token::List(list)
                },
                ")" => Token::Nil,
                _ => {
                    if s.starts_with("\""){
                        Token::String(s[1..s.len() - 1].to_string())
                    } else {
                        if let Ok(f) = s.parse::<f64>() {
                            Token::Number(f)
                        } else {
                            Token::Symbol(s.to_string())
                        }
                    }
                },
            }
        }
        None => return Err("Invalid syntax"),
    };
    Ok(token)
}

fn parse_line(line: &str) -> Result<Token, &str>{
    let preprocessed = line.trim().replace("(", " ( ").replace(")", " ) ");
    let tokens = preprocessed.trim().split_whitespace().map(|s| s.trim()).collect::<Vec<&str>>();
    let mut iter = tokens.iter();
    parse(&mut iter)
}

#[test]
fn test_lisp(){
    let token = parse_line("(a (b c) d ((e)))").unwrap();
    println!("{:?}", token);
    match token {
        Token::List(v) => assert_eq!(v.len(), 4),
        _ => panic!()
    }
}
