use super::*;

#[test]
fn test_addition() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(10));
    vm.push(Instruction::Push(20));
    vm.push(Instruction::Add);
    vm.push(Instruction::End);

    vm.run();

    assert_eq!(vm.stack.last(), Some(&30));
}

#[test]
fn test_multiplication_and_subtraction() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(10));
    vm.push(Instruction::Push(2));
    vm.push(Instruction::Mult);
    vm.push(Instruction::Push(5));
    vm.push(Instruction::Sub); // 10*2 - 5 = 15
    vm.push(Instruction::End);

    vm.run();

    assert_eq!(vm.stack.last(), Some(&15));
}

#[test]
fn test_store_and_load() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(42));
    vm.push(Instruction::Store("x".to_string()));
    vm.push(Instruction::Load("x".to_string()));
    vm.push(Instruction::Push(8));
    vm.push(Instruction::Add); // 42 + 8 = 50
    vm.push(Instruction::End);

    vm.run();

    assert_eq!(vm.stack.last(), Some(&50));
}

#[test]
fn test_division() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(100));
    vm.push(Instruction::Push(4));
    vm.push(Instruction::Div); // 100 / 4 = 25
    vm.push(Instruction::End);

    vm.run();

    assert_eq!(vm.stack.last(), Some(&25));
}

#[test]
#[should_panic]
fn test_division_by_zero_should_panic() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(10));
    vm.push(Instruction::Push(0));
    vm.push(Instruction::Div); // will panic

    vm.run();
}

#[test]
fn test_reset_clears_instructions() {
    let mut vm = VirtualMachine::new();
    vm.push(Instruction::Push(1));
    vm.push(Instruction::Push(2));
    vm.push(Instruction::Add);
    assert_eq!(vm.instructions.len(), 3);

    vm.reset();
    assert_eq!(vm.instructions.len(), 0);
    assert_eq!(vm.instruction_pointer, 0);
}
