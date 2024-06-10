use std::io::{self, Write};
use std::iter::Peekable;
use std::rc::Rc;
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    LParen,
    RParen,
    Mul,
    Add,
    Div,
    Sub,
    Num(i64),
}

#[derive(Debug)]
struct AddSub {
    md: Rc<MulDiv>,
    asp: Option<Rc<AddSubP>>,
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
    md: Rc<MulDiv>,
    asp: Option<Rc<AddSubP>>,
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
    f: Rc<Factor>,
    mdp: Option<Rc<MulDivP>>,
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
    f: Rc<Factor>,
    mdp: Option<Rc<MulDivP>>,
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
    Expr(Rc<AddSub>),
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
            '*' => Mul,
            '/' => Div,
            '-' => Sub,
            '+' => Add,
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

fn parse<'a>(toks: &mut Tokens<'a>) -> Rc<AddSub> {
    add_sub(toks)
}

fn add_sub<'a>(toks: &mut Tokens<'a>) -> Rc<AddSub> {
    if let Some(_) = toks.peek() {
        let md = mul_div(toks);
        let asp = add_sub_p(toks);
        return AddSub { md, asp }.into();
    } else {
        panic!("unexpected EOF addsub");
    }
}

fn mul_div<'a>(toks: &mut Tokens<'a>) -> Rc<MulDiv> {
    if let Some(_) = toks.peek() {
        let f = factor(toks);
        let mdp = mul_div_p(toks);
        return MulDiv { f, mdp }.into();
    } else {
        panic!("unexpected EOF addsub");
    }
}

fn add_sub_p<'a>(toks: &mut Tokens<'a>) -> Option<Rc<AddSubP>> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
            Add => {
                toks.next();
                let is_add = true;
                let md = mul_div(toks);
                let asp = add_sub_p(toks);
                return Some(AddSubP { is_add, md, asp }.into());
            }
            Sub => {
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

fn mul_div_p<'a>(toks: &mut Tokens<'a>) -> Option<Rc<MulDivP>> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
            Mul => {
                let is_mul = true;
                toks.next();
                let f = factor(toks);
                let mdp = mul_div_p(toks);
                return Some(MulDivP { is_mul, f, mdp }.into());
            }
            Div => {
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

fn factor<'a>(toks: &mut Tokens<'a>) -> Rc<Factor> {
    if let Some(t) = toks.peek() {
        use Token::*;
        match t {
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

fn main() {
    print!("Expr: ");
    // Flush to ensure the prompt is printed before user input
    io::stdout().flush().unwrap();

    // Create a new String to store the input
    let mut input = String::new();

    // Read the input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove the trailing newline
    let input = input.trim();
    let tokens = lex(&input);
    let tree = parse(&mut tokens.iter().peekable());

    // Print the input
    println!("Parse tree: {:#?}", tree);
    println!("Result: {}", tree.eval());
}
