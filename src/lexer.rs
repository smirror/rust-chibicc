use core::borrow::Borrow;
use std::io::{self, Write};
use std::mem;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .ok()
            .expect("failed to read line");

        if code == "exit\n" {
            break;
        }

        let lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(lexer);

        let expr = parser.parse();

        if let Some(expr) = expr {
            println!("{}", eval(expr.borrow()));
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(u64), // [0-9][0-9]*
    Plus,        // '+'
    Minus,       // '-'
    Times,       // '*'
    Divides,     // '/'
    LParen,      // '('
    RParen,      // ')'
}

struct Lexer {
    // 入力された文字列
    input: Vec<char>,
    // 文字列の解析中のインデックス
    position: usize,
}

impl Lexer {
    // 初期化
    fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }
    // 現在解析中の文字を字句として取得し、インデックスを一つ進める
    fn token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;
        // 空白をスキップする
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        // 現在解析中の文字を取得して字句に変換する
        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            // 数字の場合
            let mut number = vec![*curr];
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
            String::from_iter(number)
                .parse::<u64>()
                .ok()
                .and_then(|n| Some(Token::Number(n)))
        } else {
            // 数字以外の場合
            match curr {
                &'+' => Some(Token::Plus),
                &'-' => Some(Token::Minus),
                &'*' => Some(Token::Times),
                &'/' => Some(Token::Divides),
                &'(' => Some(Token::LParen),
                &')' => Some(Token::RParen),
                _ => None,
            }
        };
        self.next();
        return token;
    }
    // 入力された文字列の解析するインデックスをひとつ進める
    fn next(&mut self) {
        self.position += 1;
    }
    // 現在解析中の文字
    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }
    // 次に解析する文字
    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }
    // 文字が数字であるかどうか
    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2".chars().collect());
    assert_eq!(lexer.token(), Some(Token::Number(1_u64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2_u64)));
    assert_eq!(lexer.token(), None);
}

fn eval(expr: &Expr) -> u64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::PrefixExpr { operator: _, right } => -eval(right),
        Expr::InfixExpr {
            left,
            operator,
            right,
        } => {
            let left = eval(left);
            let right = eval(right);
            match operator.as_str() {
                "Plus" => left + right,
                "Minus" => left - right,
                "Times" => left * right,
                "Divides" => left / right,
                _ => panic!("invalid operator"),
            }
        }
    }
}

#[test]
fn test_eval() {
    do_eval("1 + 2", 3_u64);
    do_eval("1 + 2 * 3", 7_u64);
    do_eval("1 + (2 + 3) * -(3 / 3)", -4_u64);
}

#[cfg(test)]
fn do_eval(input: &str, expect: u64) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let result = eval(parser.parse().unwrap().borrow());
    assert_eq!(result, expect);
}
