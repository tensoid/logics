#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Negate,
    PinIndex (i32),
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
    InPinIndex,
}

fn lex(s: &str) -> Result<Vec<Token>, &'static str> {
    
    let mut state: ParseState = ParseState::AnyExpected;
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_pin_index: String = String::new();

    for c in s.chars() {

        if c.is_whitespace() {
            continue;
        }

        if state == ParseState::InPinIndex {
            let end_pin_index = !c.is_digit(10);

            if(end_pin_index) {
                state = ParseState::AnyExpected;

                let pin_index: i32 = current_pin_index.parse().unwrap();
                tokens.push(Token::PinIndex(pin_index));
                current_pin_index = String::new();
            }
            else {
                current_pin_index.push(c);
            }
        }

        if state == ParseState::AnyExpected {
            match c {
                '|' => tokens.push(Token::BinaryOp(BinaryOp::Or)),
                '&' => tokens.push(Token::BinaryOp(BinaryOp::And)),
                '(' => tokens.push(Token::OpenBracket),
                ')' => tokens.push(Token::CloseBracket),
                '!' => tokens.push(Token::Negate),
                _ => {
                    state = ParseState::InPinIndex;
                    current_pin_index.push(c);
                }
            }
        }
    }

    if !current_pin_index.is_empty() {
        let pin_index: i32 = current_pin_index.parse().unwrap();
        tokens.push(Token::PinIndex(pin_index));
    }

    Ok(tokens)
}

#[cfg(test)]
#[test]
fn test_lexer() {
    let mut tokens = lex("0 & !(1 | !2)").unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::PinIndex(0),
            Token::BinaryOp(BinaryOp::And),
            Token::Negate,
            Token::OpenBracket,
            Token::PinIndex(1),
            Token::BinaryOp(BinaryOp::Or),
            Token::Negate,
            Token::PinIndex(2),
            Token::CloseBracket
        ]
    );

    tokens = lex("(0 & 43) | !22").unwrap();
    assert_eq!(
        tokens,
        vec![
            Token::OpenBracket,
            Token::PinIndex(0),
            Token::BinaryOp(BinaryOp::And),
            Token::PinIndex(43),
            Token::CloseBracket,
            Token::BinaryOp(BinaryOp::Or),
            Token::Negate,
            Token::PinIndex(22)
        ]
    );
}