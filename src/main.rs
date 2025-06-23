mod interp;
mod lexers;
use interp::Instruction;
use lexers::Token;

fn main() {
    // get user input from first arg
    let input = std::env::args().nth(1).expect("No input provided");
    let lexer = lexers::Lexer::new(&input);
    let mut vm = interp::VirtualMachine::new();

    for token in lexer {
        match token {
            Token::ADD => {
                vm.add_instruction(Instruction::ADD);
            }
            Token::MULT => {
                vm.add_instruction(Instruction::MULT);
            }
            Token::Number(n) => {
                vm.stack_push(n);
            }
        }
    }

    vm.add_instruction(Instruction::PRINT);
    vm.add_instruction(Instruction::END);
    vm.run();
}
