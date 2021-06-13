extern crate rust_chibicc;
use rust_chibicc::strtol;
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

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!("Usage: rust-chibicc <code>");
        return;
    }

    let tokens = tokenize(args.nth(1).unwrap());

    // The first token must be a number
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    // Verify that the given expression starts with a number,
    // and then emit the first `mov` instruction.
    if tokens[0].ty != TokenType::Number as u64 {
        error_at("expected a number", &tokens, 0);
    }
    print!("  mov rax, {}\n", tokens[0].val);

    // Emit assembly as we consume the sequence of `+ <number>`
    // or `- <number>`.
    let mut i = 1;
    while i != tokens.len() {
        if tokens[i].ty == '+' as u64 {
            i += 1;
            if tokens[i].ty != TokenType::Number as u64 {
                error_at("expected a number", &tokens, i);
            }
            print!("  add rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        if tokens[i].ty == '-' as u64 {
            i += 1;
            if tokens[i].ty != TokenType::Number as u64 {
                error_at("expected a number", &tokens, i);
            }
            print!("  sub rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        error_at("invalid token", &tokens, i);
    }

    print!("  ret\n");
}
