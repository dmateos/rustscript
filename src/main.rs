mod interp;
mod lexers;
mod parser;
use interp::Instruction;

fn main() {
    // get user input from first arg
    let input = std::env::args().nth(1).expect("No input provided");
    let lexer = lexers::Lexer::new(&input);
    let mut vm = interp::VirtualMachine::new();
    let expressions = parser::split_into_expressions(lexer.collect());

    for e in expressions {
        let parsed = parser::eval_numerical_expression(e);
        for p in parsed {
            vm.add_instruction(p);
        }
        vm.add_instruction(Instruction::PRINT);
        vm.add_instruction(Instruction::END);
        //vm.print_instructions();
        vm.run();
        vm.clear_instructions();
    }
}
