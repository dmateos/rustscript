mod interp;
mod lexers;
mod parser;
use interp::Instruction;

fn main() {
    // get user input from first arg
    let input = std::env::args().nth(1).expect("No input provided");
    let lexer = lexers::Lexer::new(&input);
    let mut vm = interp::VirtualMachine::new();
    let parsed = parser::eval_numerical_expression(lexer.collect());

    for n in parsed {
        vm.add_instruction(n);
    }

    vm.add_instruction(Instruction::PRINT);
    vm.add_instruction(Instruction::END);
    vm.run();
}
