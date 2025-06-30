use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Add,
    Mult,
    Print,
    End,
    Push(i64),
    Load(String),
    Store(String, i64),
}

pub struct VirtualMachine {
    stack: Vec<i64>,
    instructions: Vec<Instruction>,
    symbol_table: HashMap<String, i64>,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            instruction_pointer: 0,
        }
    }

    fn advance(&mut self) {
        self.instruction_pointer += 1;
    }

    pub fn run(&mut self) {
        loop {
            match &self.instructions[self.instruction_pointer] {
                Instruction::Add => {
                    let var1 = self.stack.pop().unwrap();
                    let var2 = self.stack.pop().unwrap();
                    //println!("adding {} and {}", var1, var2);
                    self.stack.push(var1 + var2);
                }
                Instruction::Mult => {
                    let var1 = self.stack.pop().unwrap();
                    let var2 = self.stack.pop().unwrap();
                    //println!("multing {} and {}", var1, var2);
                    self.stack.push(var1 * var2);
                }
                Instruction::Print => {
                    let var = self.stack.pop().unwrap_or(0);
                    println!("{}", var);
                }
                Instruction::End => {
                    //println!("stopping ");
                    break;
                }
                Instruction::Push(n) => {
                    //println!("pushing {}", n);
                    self.stack.push(*n);
                }
                Instruction::Load(s) => {
                    self.stack.push(*self.symbol_table.get(s).unwrap());
                }
                Instruction::Store(s, v) => {
                    self.symbol_table.insert(s.to_string(), *v);
                }
            }
            self.advance();
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn reset(&mut self) {
        self.instructions.clear();
        self.instruction_pointer = 0;
    }

    #[allow(dead_code)]
    pub fn print_instructions(&self) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("{}: {:?}", i, instruction);
        }
    }
}
