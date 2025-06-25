mod interp;
mod lexers;
mod parser;
use interp::Instruction;

fn main() {
    // get user input from first arg
    let input = std::env::args().nth(1).expect("No input provided");
    let lexer = lexers::Lexer::new(&input);
    let mut vm = interp::VirtualMachine::new();
    let expressions = parser::split_into_statements(lexer.collect());

    vm.push(Instruction::STORE("x".to_string(), 10));

    for e in expressions {
        let parsed = parser::parse_expression(e);
        for p in parsed {
            vm.push(p);
        }
        vm.push(Instruction::PRINT);
        vm.push(Instruction::END);
        vm.print_instructions();
        vm.run();
        vm.reset();
    }
}
