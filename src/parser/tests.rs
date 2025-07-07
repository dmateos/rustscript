use super::*;


#[test]
fn test_split_into_statements() {
    let tokens = vec![
        Token::Number(1), Token::Add, Token::Number(2), Token::Semicolon,
        Token::Ident("x".into()), Token::Assign, Token::Number(3), Token::Semicolon,
    ];
    let stmts = split_into_statements(tokens);
    assert_eq!(stmts.len(), 2);
    assert_eq!(stmts[0], vec![Token::Number(1), Token::Add, Token::Number(2)]);
    assert_eq!(stmts[1], vec![Token::Ident("x".into()), Token::Assign, Token::Number(3)]);
}

#[test]
fn test_parse_expression_simple_add() {
    let tokens = vec![Token::Number(2), Token::Add, Token::Number(3)];
    let instrs = parse_expression(tokens);
    assert_eq!(instrs, vec![Instruction::Push(2), Instruction::Push(3), Instruction::Add]);
}

#[test]
fn test_parse_expression_precedence() {
    let tokens = vec![Token::Number(2), Token::Add, Token::Number(3), Token::Mult, Token::Number(4)];
    let instrs = parse_expression(tokens);
    assert_eq!(instrs, vec![Instruction::Push(2), Instruction::Push(3), Instruction::Push(4), Instruction::Mult, Instruction::Add]);
}

#[test]
fn test_parse_expression_with_parentheses() {
    let tokens = vec![
        Token::OpenParen, Token::Number(2), Token::Add, Token::Number(3), Token::CloseParen, Token::Mult, Token::Number(4)
    ];
    let instrs = parse_expression(tokens);
    assert_eq!(instrs, vec![Instruction::Push(2), Instruction::Push(3), Instruction::Add, Instruction::Push(4), Instruction::Mult]);
}

#[test]
fn test_parse_assignment() {
    let tokens = vec![Token::Ident("x".into()), Token::Assign, Token::Number(7), Token::Add, Token::Number(5)];
    let instrs = parse_statement(&tokens);
    assert_eq!(
        instrs,
        vec![Instruction::Push(7), Instruction::Push(5), Instruction::Add, Instruction::Store("x".into())]
    );
}

#[test]
fn test_parse_expression_with_variable_load() {
    let tokens = vec![Token::Ident("a".into()), Token::Add, Token::Number(1)];
    let instrs = parse_expression(tokens);
    assert_eq!(instrs, vec![Instruction::Load("a".into()), Instruction::Push(1), Instruction::Add]);
}