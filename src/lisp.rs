use std::fmt::{Display, Formatter};
use std::io;
use core::slice::{Iter};

enum Token {
    Nil,
    Single(String),
    List(Vec<Box<Token>>),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Nil => write!(f, "()"),
            Token::Single(s) => write!(f, "{}", s),
            Token::List(v) => {
                write!(f, "(")?;
                let mut space = false;
                for b in v {
                    if space {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", *b)?;
                    space = true;
                }
                write!(f, ")")
            },
        }
    }
}

fn parse(token_iterator: &mut Iter<&str>) -> Token {
    match token_iterator.next() {
        Some(s) => match *s {
            "(" => {
                let mut v: Vec<Box<Token>> = vec![];
                loop {
                    let token = parse(token_iterator);
                    match token {
                        Token::Nil => break,
                        _ => v.push(Box::new(token))
                    }
                }
                Token::List(v)
            },
            ")" => Token::Nil,
            _ => Token::Single(s.to_string()),
        }
        None => Token::Nil,
    }
}

fn read_eval() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let clean = line.replace("(", " ( ").replace(")", " ) ");
        let clean = clean.trim();
        let tokens: Vec<&str> = clean.split(" ").filter(|t| t.trim().len() > 0).collect();
        if tokens.len() == 0 {
            break;
        }
        let mut token_iterator = &mut tokens.iter();
        let expression = parse( token_iterator);
        println!("{}", expression);
    }
}
