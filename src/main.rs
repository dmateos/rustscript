mod interp;
mod lexers;
mod parser;
use interp::Instruction;

fn main() {
    // get user input from first arg
    let input = std::env::args().nth(1).expect("No input provided");
    let lexer = lexers::Lexer::new(&input);
    let mut vm = interp::VirtualMachine::new();

    let statements = parser::split_into_statements(lexer.collect());
    for e in statements {
        let parsed = parser::parse_statement(&e);
        for p in parsed {
            vm.push(p);
        }
        vm.push(Instruction::Print);
        vm.push(Instruction::End);
        vm.print_instructions();
        vm.run();
        vm.reset();
    }
}
