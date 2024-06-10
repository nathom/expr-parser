use std::io::{self, Write};
use std::iter::Peekable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        assert_eq!(eval("2 + 2"), 4);
    }

    #[test]
    fn test_simple_subtraction() {
        assert_eq!(eval("5 - 3"), 2);
    }

    #[test]
    fn test_simple_multiplication() {
        assert_eq!(eval("3 * 4"), 12);
    }

    #[test]
    fn test_simple_division() {
        assert_eq!(eval("10 / 2"), 5);
    }

    #[test]
    fn test_combined_expression() {
        assert_eq!(eval("2 + 3 * 4"), 14);
    }

    #[test]
    fn test_combined_expression_with_parentheses() {
        assert_eq!(eval("(2 + 3) * 4"), 20);
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(eval("((1 + 2) * (3 + 4))"), 21);
    }

    #[test]
    fn test_division_and_subtraction() {
        assert_eq!(eval("20 / 4 - 2"), 3);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(eval("1000000 * 2 + 3000000 / 1000"), 2003000);
    }

    #[test]
    fn test_expression_with_spaces() {
        assert_eq!(eval(" 2 + 2 "), 4);
    }

    #[test]
    fn test_simple_addition_negative() {
        assert_eq!(eval("2 + -2"), 0);
    }

    #[test]
    fn test_simple_subtraction_negative() {
        assert_eq!(eval("5 - -3"), 8);
    }

    #[test]
    fn test_simple_multiplication_negative() {
        assert_eq!(eval("3 * -4"), -12);
    }

    #[test]
    fn test_simple_division_negative() {
        assert_eq!(eval("10 / -2"), -5);
    }

    #[test]
    fn test_combined_expression_negative() {
        assert_eq!(eval("2 + 3 * -4"), -10);
    }

    #[test]
    fn test_combined_expression_with_parentheses_negative() {
        assert_eq!(eval("(2 + -3) * 4"), -4);
    }

    #[test]
    fn test_nested_parentheses_negative() {
        assert_eq!(eval("((1 + -2) * (3 + 4))"), -7);
    }

    #[test]
    fn test_division_and_subtraction_negative() {
        assert_eq!(eval("20 / -4 - 2"), -7);
    }

    #[test]
    fn test_large_numbers_negative() {
        assert_eq!(eval("1000000 * 2 + -3000000 / 1000"), 1997000);
    }

    #[test]
    fn test_negative_numbers_negative() {
        assert_eq!(eval("-5 + -3"), -8);
    }

    #[test]
    fn test_negative_result_negative() {
        assert_eq!(eval("3 - -5"), 8);
    }

    #[test]
    fn test_negative_multiplication_negative() {
        assert_eq!(eval("-3 * -4"), 12);
    }

    #[test]
    fn test_negative_division_negative() {
        assert_eq!(eval("-10 / -2"), 5);
    }

    #[test]
    fn test_expression_with_spaces_negative() {
        assert_eq!(eval(" 2 + -2 "), 0);
    }

    #[test]
    fn test_subtracting_negative_numbers_negative() {
        assert_eq!(eval("5 - -3"), 8);
    }

    #[test]
    fn test_multiplying_negative_numbers_negative() {
        assert_eq!(eval("-5 * -3"), 15);
    }

    #[test]
    fn test_dividing_negative_numbers_negative() {
        assert_eq!(eval("-6 / -2"), 3);
    }

    #[test]
    fn test_expression_with_multiple_operations_negative() {
        assert_eq!(eval("2 + 3 * -4 - 6 / 3"), -12);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    LParen,
    RParen,
    Star,
    Plus,
    Slash,
    Minus,
    Num(i64),
}

#[derive(Debug)]
struct AddSub {
    md: Box<MulDiv>,
    asp: Option<Box<AddSubP>>,
}

impl AddSub {
    fn eval(&self) -> i64 {
        let m = self.md.eval();
        if let Some(e) = &self.asp {
            return e.eval(m);
        }
        return m;
    }
}

#[derive(Debug)]
struct AddSubP {
    is_add: bool,
    md: Box<MulDiv>,
    asp: Option<Box<AddSubP>>,
}

impl AddSubP {
    fn eval(&self, n: i64) -> i64 {
        let m = if self.is_add {
            n + self.md.eval()
        } else {
            n - self.md.eval()
        };
        if let Some(e) = &self.asp {
            return e.eval(m);
        } else {
            return m;
        }
    }
}

#[derive(Debug)]
struct MulDiv {
    f: Box<Factor>,
    mdp: Option<Box<MulDivP>>,
}

impl MulDiv {
    fn eval(&self) -> i64 {
        let m = self.f.eval();
        if let Some(e) = &self.mdp {
            return e.eval(m);
        }
        return m;
    }
}

#[derive(Debug)]
struct MulDivP {
    is_mul: bool,
    f: Box<Factor>,
    mdp: Option<Box<MulDivP>>,
}

impl MulDivP {
    fn eval(&self, n: i64) -> i64 {
        let m = if self.is_mul {
            n * self.f.eval()
        } else {
            n / self.f.eval()
        };
        if let Some(e) = &self.mdp {
            return e.eval(m);
        } else {
            return m;
        }
    }
}

#[derive(Debug)]
enum Factor {
    Num(i64),
    Expr(Box<AddSub>),
}

impl Factor {
    fn eval(&self) -> i64 {
        use Factor::*;
        match self {
            Num(i) => *i,
            Expr(e) => e.eval(),
        }
    }
}

fn lex(s: &str) -> Vec<Token> {
    use Token::*;
    let chars = &mut s.chars().peekable();
    let mut ret = Vec::new();
    while let Some(c) = chars.next() {
        let tok = match c {
            '(' => LParen,
            ')' => RParen,
            '*' => Star,
            '/' => Slash,
            '-' => Minus,
            '+' => Plus,
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap();
                while let Some(cd) = chars.peek() {
                    if cd.is_digit(10) {
                        num = num * 10 + cd.to_digit(10).unwrap();
                        chars.next();
                    } else {
                        break;
                    }
                }
                Num(num as i64)
            }
            c if c.is_whitespace() => continue,
            _ => panic!("Invalid token"),
        };
        ret.push(tok);
    }
    return ret;
}

type Tokens<'a> = Peekable<std::slice::Iter<'a, Token>>;

fn parse<'a>(toks: &mut Tokens<'a>) -> Box<AddSub> {
    add_sub(toks)
}

fn add_sub<'a>(toks: &mut Tokens<'a>) -> Box<AddSub> {
    if let Some(_) = toks.peek() {
        let md = mul_div(toks);
        let asp = add_sub_p(toks);
        return AddSub { md, asp }.into();
    } else {
        panic!("unexpected EOF addsub");
    }
}

fn mul_div<'a>(toks: &mut Tokens<'a>) -> Box<MulDiv> {
    if let Some(_) = toks.peek() {
        let f = factor(toks);
        let mdp = mul_div_p(toks);
        return MulDiv { f, mdp }.into();
    } else {
        panic!("unexpected EOF addsub");
    }
}

fn add_sub_p<'a>(toks: &mut Tokens<'a>) -> Option<Box<AddSubP>> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
            Plus => {
                toks.next();
                let is_add = true;
                let md = mul_div(toks);
                let asp = add_sub_p(toks);
                return Some(AddSubP { is_add, md, asp }.into());
            }
            Minus => {
                toks.next();
                let is_add = false;
                let md = mul_div(toks);
                let asp = add_sub_p(toks);
                return Some(AddSubP { is_add, md, asp }.into());
            }
            _ => return None,
        }
    } else {
        return None;
    }
}

fn mul_div_p<'a>(toks: &mut Tokens<'a>) -> Option<Box<MulDivP>> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
            Star => {
                let is_mul = true;
                toks.next();
                let f = factor(toks);
                let mdp = mul_div_p(toks);
                return Some(MulDivP { is_mul, f, mdp }.into());
            }
            Slash => {
                let is_mul = false;
                toks.next();
                let f = factor(toks);
                let mdp = mul_div_p(toks);
                return Some(MulDivP { is_mul, f, mdp }.into());
            }
            _ => return None,
        }
    } else {
        return None;
    }
}

fn factor<'a>(toks: &mut Tokens<'a>) -> Box<Factor> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
            Minus => {
                toks.next();
                if let Some(Num(n)) = toks.next() {
                    return Factor::Num(n * -1).into();
                } else {
                    panic!("Expected number after - sign");
                }
            }
            Num(n) => {
                toks.next();
                return Factor::Num(*n).into();
            }
            LParen => {
                toks.next();
                let asp = add_sub(toks);
                let rp = toks.next().unwrap();
                if rp != &RParen {
                    panic!("Expected rparen")
                }
                return Factor::Expr(asp).into();
            }
            _ => panic!("Unexpected in factor {:?}", t),
        }
    } else {
        panic!("Unexpected EOF in factor");
    }
}

fn eval(s: &str) -> i64 {
    let tokens = lex(&s);
    let tree = parse(&mut tokens.iter().peekable());
    tree.eval()
}

fn main() {
    print!("Expr: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let tokens = lex(&input);
    let tree = parse(&mut tokens.iter().peekable());

    // Print the input
    println!("Parse tree: {:#?}", tree);
    println!("Result: {}", eval(input));
}
