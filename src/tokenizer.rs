extern crate combine;
extern crate combine_language;

use combine::char::{alpha_num, letter, string};
use combine::{chainl1, parser, satisfy, ParseResult, Parser, Stream};
use combine_language::{Identifier, LanguageDef, LanguageEnv};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Divides(Box<Expr>, Box<Expr>),
}

fn calc_env<'a, I>() -> LanguageEnv<'a, I>
where
    I: Stream<Item = char>,
    I: 'a,
{
    LanguageEnv::new(LanguageDef {
        ident: Identifier {
            start: letter(),
            rest: alpha_num(),
            reserved: ["if", "then", "else", "let", "in", "type"]
                .iter()
                .map(|x| (*x).into())
                .collect(),
        },
        op: Identifier {
            start: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            rest: satisfy(|c| "+-*/".chars().any(|x| x == c)),
            reserved: ["+", "-", "*", "/"].iter().map(|x| (*x).into()).collect(),
        },
        comment_start: string("/*").map(|_| ()),
        comment_end: string("*/").map(|_| ()),
        comment_line: string("//").map(|_| ()),
    })
}

// 整数または括弧で括られた式
fn factor<I>(input: I) -> ParseResult<Box<Expr>, I>
where
    I: Stream<Item = char>,
{
    let env = calc_env();
    let number = env.integer().map(|x| Box::new(Expr::Number(x)));
    let parenthesized = env.parens(parser(expr));
    number.or(parenthesized).parse_stream(input)
}

// 掛け算・割り算またはfactor
fn term<I>(input: I) -> ParseResult<Box<Expr>, I>
where
    I: Stream<Item = char>,
{
    let env = calc_env();
    let op = env.reserved_op("*").or(env.reserved_op("/")).map(|op| {
        move |lhs, rhs| {
            if op == "*" {
                Box::new(Expr::Times(lhs, rhs))
            } else if op == "/" {
                Box::new(Expr::Divides(lhs, rhs))
            } else {
                unreachable!()
            }
        }
    });
    chainl1(parser(factor), op).parse_stream(input)
}

// 全ての式
fn expr<I>(input: I) -> ParseResult<Box<Expr>, I>
where
    I: Stream<Item = char>,
{
    let env = calc_env();
    let op = env.reserved_op("+").or(env.reserved_op("-")).map(|op| {
        move |lhs, rhs| {
            if op == "+" {
                Box::new(Expr::Plus(lhs, rhs))
            } else if op == "-" {
                Box::new(Expr::Minus(lhs, rhs))
            } else {
                unreachable!()
            }
        }
    });
    chainl1(parser(term), op).parse_stream(input)
}

fn main() {
    let mut parser = parser(expr);
    println!("{:?}", parser.parse("1 + 2 * 3"));
}
