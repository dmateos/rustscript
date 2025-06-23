pub enum Instruction {
    ADD = 0,
    MULT,
    PRINT,
    END,
}

pub struct VirtualMachine {
    stack: Vec<i64>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            instructions: Vec::new(),
            instruction_pointer: 0,
        }
    }

    fn advance(&mut self) {
        self.instruction_pointer += 1;
    }

    pub fn run(&mut self) {
        loop {
            match self.instructions[self.instruction_pointer] {
                Instruction::ADD => {
                    let var1 = self.stack.pop().unwrap();
                    let var2 = self.stack.pop().unwrap();
                    println!("adding {} and {}", var1, var2);
                    self.stack.push(var1 + var2);
                }
                Instruction::MULT => {
                    let var1 = self.stack.pop().unwrap();
                    let var2 = self.stack.pop().unwrap();
                    println!("multing {} and {}", var1, var2);
                    self.stack.push(var1 * var2);
                }
                Instruction::PRINT => {
                    let var = self.stack.pop().unwrap();
                    println!("{}", var);
                }
                Instruction::END => {
                    break;
                }
            }
            self.advance();
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn stack_push(&mut self, value: i64) {
        self.stack.push(value);
    }
}
