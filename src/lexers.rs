#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Mult,
    Div,
    Sub,
    Assign,
    Semicolon,
    OpenParen,
    CloseParen,
    Ident(String),
    Number(i64),
    Function(String),
}

pub struct Lexer {
    position: usize,
    input: Vec<char>,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            position: 0,
            input: input.chars().collect(),
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => {
                    return Some(Token::Number(self.lex_number().unwrap()));
                }
                'A'..='z' => {
                    return Some(self.lex_identifier());
                }
                '+' => {
                    self.advance();
                    return Some(Token::Add);
                }
                '*' => {
                    self.advance();
                    return Some(Token::Mult);
                }
                '/' => {
                    self.advance();
                    return Some(Token::Div);
                }
                '-' => {
                    self.advance();
                    return Some(Token::Sub);
                }
                ';' => {
                    self.advance();
                    return Some(Token::Semicolon);
                }
                '=' => {
                    self.advance();
                    return Some(Token::Assign);
                }
                '(' => {
                    self.advance();
                    return Some(Token::OpenParen);
                }
                ')' => {
                    self.advance();
                    return Some(Token::CloseParen);
                }
                _ => {
                    self.advance();
                }
            }
        }
        Option::None
    }

    fn lex_number(&mut self) -> Result<i64, std::num::ParseIntError> {
        let start = self.position;

        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        let numstr: String = self.input[start..self.position].iter().collect();
        numstr.parse::<i64>()
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.position;

        while let Some(ch) = self.peek() {
            match ch {
                'A'..='z' => {
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
        let identstr: String = self.input[start..self.position].iter().collect();
        self.match_reserved(&identstr)
    }

    fn match_reserved(&self, ident: &str) -> Token {
        match ident {
            "func" => Token::Function(ident.to_string()),
            _ => Token::Ident(ident.to_string())
        }
    }
}
