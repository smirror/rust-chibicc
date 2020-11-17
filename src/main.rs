#[derive(Debug)]
pub enum Token {
    Number(u64), // [0-9][0-9]*
    Plus,        // '+'
    Minus,       // '-'
    Times,       // '*'
    Divides,     // '/'
    LParen,      // '('
    RParen,      // ')'
}

impl Token {
    /// あとで使う便利関数。
    pub fn expect_num(&self) -> u64 {
        match self {
            Token::Number(n) => *n,
            t => panic!("Expect number but found {:?}", t),
        }
    }
}

/// トークンのイテレータを表す構造体
pub struct TokenIter<'a> {
    s: &'a str,
}

/// 文字列を受け取って、トークンのイテレータを返す関数。つまりトークナイザー。
/// Rustのイテレータは遅延評価なのでここでは何もしていない。
pub fn tokenize<'a>(s: &'a str) -> TokenIter<'a> {
    TokenIter { s }
}

/// トークナイザーの中身。
/// やっていることは、次のトークンの判定を行い、内部の文字列を更新するだけ。
impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start();
        if self.s.is_empty() {
            return None;
        }
        match self.s.as_bytes()[0] {
            b'+' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Plus);
            }
            b'-' => {
                self.s = self.s.split_at(1).1;
                return Some(Token::Minus);
            }
            _ => {}
        }

        let (digit_s, remain_s) = split_digit(self.s);
        if !digit_s.is_empty() {
            self.s = remain_s;
            return Some(Token::Number(u64::from_str_radix(digit_s, 10).unwrap()));
        }

        panic!("Invalid token stream")
    }
}

/// Rustにstrtol関数がないので同じような挙動をする関数を定義する。
fn split_digit(s: &str) -> (&str, &str) {
    let first_non_num_idx = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
    s.split_at(first_non_num_idx)
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let mut tokens = tokenize(arg.as_str());
    // The first token must be a number
    println!(".intel_syntax noprefix");
    println!(".global main");

    // ... followed by either `+ <number>` or `- <number>`.
    println!("main:");

    println!("   mov rax, {}", tokens.next().unwrap().expect_num());

    while let Some(token) = tokens.next() {
        let n = tokens.next().unwrap().expect_num();
        match token {
            Token::Plus => println!("   add rax, {}", n),
            Token::Minus => println!("   sub rax, {}", n),
            _ => panic!("Unexpected Operator"),
        }
    }

    println!("   ret");
}
