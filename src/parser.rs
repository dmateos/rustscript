use crate::interp::Instruction;
use crate::lexers::Token;

fn op_order(op: &Token) -> u8 {
    match op {
        Token::ADD => 1,
        Token::MULT => 2,
        _ => 0,
    }
}

fn to_instruction(token: &Token) -> Instruction {
    match token {
        Token::ADD => Instruction::ADD,
        Token::MULT => Instruction::MULT,
        _ => panic!("Not an operator"),
    }
}

pub fn eval_numerical_expression(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut opq = Vec::new();
    let mut outq = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                outq.push(Instruction::PUSH(n));
            }
            Token::ADD | Token::MULT => {
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
        }
    }

    while let Some(op) = opq.pop() {
        outq.push(to_instruction(&op));
    }
    outq
}
