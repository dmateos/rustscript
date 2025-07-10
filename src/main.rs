mod interp;
mod lexers;
mod parser;
use interp::Instruction;

fn main() {
    // get user input from first arg
    let mut vm = interp::VirtualMachine::new();

    loop { 
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let lexer = lexers::Lexer::new(&input);

        let statements = parser::split_into_statements(lexer.collect());
        for e in statements {
            let parsed = parser::parse_statement(&e);
            for p in parsed {
                vm.push(p);
            }
            vm.push(Instruction::End);
            //vm.print_instructions();
            vm.run();
            vm.reset();
        }
    }
}
