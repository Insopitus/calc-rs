use std::str::Chars;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token{
    Num(u32),
    Op(Operator),
    Paren(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator{
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Calculator{
}

impl Calculator {
    pub fn parse<T:AsRef<str>>(expr:T)->Result<f64,Error>{
        let expr = expr.as_ref();
        let chars = expr.chars();
        let tokens = Calculator::tokenization(chars)?;
        let expr = Calculator::expression(tokens);
        Ok(Calculator::excute(expr))
    }
    fn tokenization(chars:Chars)->Result<Vec<Token>,Error>{
        let mut tokens = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9'=>match tokens.last_mut(){
                    Some(Token::Num(n))=>{
                        *n = *n*10 +(c as u32-48);
                    },
                    _=>{
                        let digit = c as u32 -48;
                        tokens.push(Token::Num(digit))
                    }
                },
                '('=>{
                    tokens.push(Token::Paren('('));
                    parens.push(c);
                },
                ')'=>{
                    tokens.push(Token::Paren(')'));
                    if let Some(p) = parens.pop(){
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    }else{
                        return Err(Error::MismatchedParens);
                    }
                }
                '+'=>tokens.push(Token::Op(Operator::Add)),
                '-'=>tokens.push(Token::Op(Operator::Sub)),
                '*'=>tokens.push(Token::Op(Operator::Mul)),
                '/'=>tokens.push(Token::Op(Operator::Div)),
                ' '=>{}, // 1 2 + 3 should be okay?
                '\n'=>{}, 
                '\r'=>{},
                _=>return Err(Error::BadToken(c))

            }
        }
        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }
        Ok(tokens)
    }
    /// Shunting Yard Algorithm
    fn expression(mut tokens:Vec<Token>)->Vec<Token>{
        tokens.reverse();
        let mut queue = Vec::new();
        let mut stack:Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token{
                Token::Num(_)=>queue.push(token),
                Token::Op(_)=>{
                    while !stack.is_empty() && stack[stack.len()-1]>=token{
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Paren('(')=>stack.push(token),
                Token::Paren(')')=>{
                    while !stack.is_empty() && stack[stack.len()-1] != Token::Paren('('){
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _=>{}

            }
        }
        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        queue
    }
    pub fn excute(expr:Vec<Token>)->f64{
        let mut stack:Vec<f64> = Vec::new();
        for token in expr {
            match token {
                Token::Num(num)=>stack.push(num as f64),
                Token::Op(operator)=>match operator {
                    Operator::Add =>{
                        let rhs = stack.pop().unwrap();
                        let lhs = stack.pop().unwrap();
                        stack.push(lhs + rhs);
                    },
                    Operator::Sub =>{
                        let rhs = stack.pop().unwrap();
                        let lhs = stack.pop().unwrap();
                        stack.push(lhs - rhs);
                    },
                    Operator::Mul=>{
                        let rhs = stack.pop().unwrap();
                        let lhs = stack.pop().unwrap();
                        stack.push(lhs * rhs);
                    },
                    Operator::Div =>{
                        let rhs = stack.pop().unwrap();
                        let lhs = stack.pop().unwrap();
                        stack.push(lhs / rhs);
                    }
                },
                Token::Paren(_)=>{}
            }

        }
        stack.pop().unwrap()
    }
}

#[derive(Debug)]
pub enum Error{
    BadToken(char),
    MismatchedParens
}

#[cfg(test)]
mod test {
    use crate::Calculator;

    #[test]
    fn basic(){        
        assert_eq!(Calculator::parse("13+4*5+3/4").unwrap(),13.0+4.0*5.0+3.0/4.0);
    }
    #[test]
    fn parenthese(){
        assert_eq!(Calculator::parse("(13+4)*5+3/4").unwrap(),(13.0+4.0)*5.0+3.0/4.0);
    }
}