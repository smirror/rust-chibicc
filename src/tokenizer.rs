pub mod tokenizer;

use std::{env, process::exit};


// Tokenizer
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(u64), // [0-9][0-9]*
    Plus,        // '+'
    Minus,       // '-'
    Times,       // '*'
    Divides,     // '/'
    LParen,      // '('
    RParen,      // ')'
}

#[derive(Default, Debug)]
struct Token {
    ty: u64,       // Token type
    val: u64,      // Number literal
    input: String, // Token string (for error reporting)
}

fn tokenize(mut p: String) -> Vec<Token> {
    // Tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // Skip whitespce
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        // + or -
        if c == '+' || c == '-' {
            let token = Token {
                ty: c as u64,
                input: org.clone(),
                ..Default::default()
            };
            p = p.split_off(1); // p++
            tokens.push(token);
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            let (n, remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Number as u64,
                input: org.clone(),
                val: n.unwrap() as u64,
            };
            tokens.push(token);
            continue;
        }

        eprintln!("cannot tokenize: {}", p);
        exit(1);
    }
    return tokens;
}


fn error_at(error_message : &str,  tokens: &Vec<Token>, i: usize) {
    eprintln!("{}", &error_message);
    eprintln!("{}", &tokens[0].input);
    for _ in 0..i {
        eprint!("{}", " ");
    }
    let err_ch = &tokens[0].input[i..i+1];
    eprintln!("{} unexpected character: {}", "^", err_ch);
    exit(1);
}
