use crate::lexer::{Lexer, Token};

#[derive(Debug)]
enum Expr {
    // 数字
    Number(u64),
    // 前置演算子式
    // 式の前に演算子のついた式
    // 例）"-10", "-(1 + 2)"
    PrefixExpr {
        operator: String,
        right: Box<Expr>,
    },
    // 中置演算子式
    // 式と式の間に演算子のある式
    // 例）"1 + 2", "3 * (4 + 5 + 6)"
    InfixExpr {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    SUM,
    PRODUCT,
    PREFIX,
}

struct Parser {
    // 字句解析器
    lexer: Lexer,
    // 現在解析中の字句
    curr: Option<Token>,
    // 次に解析する字句
    peek: Option<Token>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { lexer, curr, peek }
    }
    fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;

        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        return Some(left);
    }
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }
    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let number = self.parse_expression(Precedence::PREFIX)?;
        return Some(Box::new(Expr::PrefixExpr {
            operator: "Minus".to_string(),
            right: number,
        }));
    }
    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
    fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::LOWEST);
        if self.is_peek(&Token::RParen) {
            self.next();
            return expression;
        } else {
            return None;
        }
    }
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Times | Token::Divides => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        return Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }));
    }
    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }
    fn is_peek(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }
    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Times | Token::Divides => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}

#[test]
fn test_parser() {
    do_parser(
        "1 + 2",
        r#"Some(InfixExpr { left: Number(1.0), operator: "Plus", right: Number(2.0) })"#,
    );
    do_parser(
        "- 1 + 2 * 3",
        r#"Some(InfixExpr { left: PrefixExpr { operator: "Minus", right: Number(1.0) }, operator: "Plus", right: InfixExpr { left: Number(2.0), operator: "Asterisk", right: Number(3.0) } })"#,
    );
}

#[cfg(test)]
fn do_parser(input: &str, expect: &str) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    assert_eq!(format!("{:?}", parser.parse()), expect);
}
