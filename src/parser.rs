use crate::interp::Instruction;
use crate::lexers::Token;

#[cfg(test)]
mod tests;

fn op_order(op: &Token) -> u8 {
    match op {
        Token::Add => 1,
        Token::Sub => 2,
        Token::Div => 3,
        Token::Mult => 4,
        _ => 0,
    }
}

fn to_instruction(token: &Token) -> Instruction {
    match token {
        Token::Add => Instruction::Add,
        Token::Mult => Instruction::Mult,
        Token::Div => Instruction::Div,
        Token::Sub => Instruction::Sub,
        _ => panic!("Not an operator {:?}", token),
    }
}

pub fn split_into_statements(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut buffer = Vec::new();
    let mut sub_buffer = Vec::new();

    for t in tokens {
        match t {
            Token::Semicolon => {
                buffer.push(sub_buffer);
                sub_buffer = Vec::new();
            }
            _ => {
                sub_buffer.push(t);
            }
        }
    }
    buffer
}

pub fn parse_statement(tokens: &[Token]) -> Vec<Instruction>  {
    match tokens {
        [Token::Ident(name), Token::Assign, rest @ ..] => {
            let mut instructions = parse_expression(rest.to_vec());
            instructions.push(Instruction::Store(name.into()));
            instructions

        }
        _ => { 
            parse_expression(tokens.to_vec())
        },
    }
}

fn parse_expression(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut opq = Vec::new();
    let mut outq = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                outq.push(Instruction::Push(n));
            }
            Token::Add | Token::Mult | Token::Div | Token::Sub => {
                while let Some(top) = opq.last() {
                    if op_order(top) >= op_order(&token) {
                        let op = opq.pop().unwrap();
                        outq.push(to_instruction(&op));
                    } else {
                        break;
                    }
                }
                opq.push(token);
            }
            Token::Ident(n) => {
                outq.push(Instruction::Load(n));
            }
            Token::OpenParen => { 
                opq.push(token);
            }
            Token::CloseParen => { 
                while let Some(top) = opq.last() {
                    if top != &Token::OpenParen {
                        let op = opq.pop().unwrap();
                        outq.push(to_instruction(&op))
                    } else {
                        opq.pop();
                        break;
                    }
                }
            }
            _ => {
                println!("Parser: Not implemented {:?}", token);
            }
        }
    }

    while let Some(op) = opq.pop() {
        outq.push(to_instruction(&op));
    }
    outq
}
