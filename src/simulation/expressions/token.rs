#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Negate,
    Pin { isInput: bool, id: String },
    BinaryOp(BinaryOp),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BinaryOp {
    And,
    Or,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParseState {
    AnyExpected,
    InPinId,
}


fn lex(s: &str) -> Result<Vec<Token>, &'static str> {
    
    let mut state: ParseState = ParseState::AnyExpected;
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_pin_id: String = String::new();

    Some(Token::BinaryOp(BinaryOp::And));

    for c in s.chars() {

        if state == ParseState::InPinId {
            let end_pin_id = match c {
                '(' | ')' | '!' => true,
                _ => false
            };

            if(end_pin_id) {
                state = ParseState::AnyExpected;
            }
        }

        if state == ParseState::AnyExpected {
            match c {
                '(' => tokens.push(Token::OpenBracket),
                ')' => tokens.push(Token::CloseBracket),
                '!' => tokens.push(Token::CloseBracket),
                _ => {
                    state = ParseState::InPinId;
                    current_pin_id.push(c);
                }
            }
        }
    }

    Ok(tokens)
}