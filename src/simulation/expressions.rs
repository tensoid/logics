use std::collections::VecDeque;

const MAX_RECURSION_DEPTH: u16 = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Negate,
    PinIndex(i32),
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

//TODO: error enums as result instead of &'static str

fn lex(s: &str) -> Result<Vec<Token>, &'static str> {
    let mut state: ParseState = ParseState::AnyExpected;
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_pin_index: String = String::new();

    for c in s.chars() {
        if state == ParseState::InPinIndex {
            let end_pin_index = !c.is_ascii_digit();

            if end_pin_index {
                state = ParseState::AnyExpected;

                let pin_index: i32 = current_pin_index.parse().unwrap();
                tokens.push(Token::PinIndex(pin_index));
                current_pin_index = String::new();
            } else {
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
                    if c.is_whitespace() {
                        continue;
                    }

                    if !c.is_ascii_digit() {
                        return Err("Unexpected character in equation string");
                    }
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

//TODO: force brackets for each bin op

#[derive(Debug, PartialEq, Eq, Clone)]
enum AstNode {
    Negate(Box<AstNode>),
    Binary(BinaryOp, Box<AstNode>, Box<AstNode>),
    PinIndex(i32),
}

impl AstNode {
    fn munch_tokens(tokens: &mut VecDeque<Token>, depth: u16) -> Result<Self, &'static str> {
        if depth == 0 {
            return Err("expression too deep");
        }

        let next = match tokens.front() {
            Some(x) => x,
            None => return Err("unexpected end of expression"),
        };
        match next {
            Token::CloseBracket => Err("Unexpected closing bracket"),
            Token::Negate => {
                tokens.remove(0);
                // Negate exactly the next token
                // !a & b -> (!a) & b
                match tokens.front() {
                    Some(Token::OpenBracket) => Ok(AstNode::Negate(Box::new(Self::munch_tokens(
                        tokens,
                        depth - 1,
                    )?))),
                    Some(Token::PinIndex(index)) => {
                        // is it like "!abc" or "!abc & xyz"
                        let negated = AstNode::Negate(Box::new(AstNode::PinIndex(*index)));
                        match tokens.get(1) {
                            Some(Token::BinaryOp(_)) => {
                                // "!abc & xyz"
                                // convert to unambiguous form and try again
                                tokens.insert(0, Token::OpenBracket);
                                tokens.insert(1, Token::Negate);
                                tokens.insert(2, Token::OpenBracket);
                                tokens.insert(4, Token::CloseBracket);
                                tokens.insert(5, Token::CloseBracket);
                                Self::munch_tokens(tokens, depth - 1)
                            }
                            None | Some(Token::CloseBracket) => {
                                // "!abc"
                                tokens.remove(0); // remove PinIndex
                                Ok(negated)
                            }
                            Some(_) => Err("invalid token after negated PinIndex"),
                        }
                    }
                    Some(Token::Negate) => Err("can't double Negate, that would be pointless"),
                    Some(_) => Err("expected expression"),
                    None => Err("Expected token to Negate, got EOF"),
                }
            }
            Token::OpenBracket => {
                tokens.remove(0); // open bracket
                let result = Self::munch_tokens(tokens, depth - 1)?;
                match tokens.remove(0) {
                    // remove closing bracket
                    Some(Token::CloseBracket) => {}
                    _ => return Err("expected closing bracket"),
                };
                // check for binary op afterwards
                return match tokens.front() {
                    Some(Token::BinaryOp(op)) => {
                        let op = *op;
                        tokens.remove(0).unwrap(); // remove binary op
                        Ok(AstNode::Binary(
                            op,
                            Box::new(result),
                            Box::new(Self::munch_tokens(tokens, depth - 1)?),
                        ))
                    }
                    Some(Token::CloseBracket) | None => Ok(result),
                    Some(_) => Err("invald token after closing bracket"),
                };
            }
            Token::BinaryOp(_) => Err("Unexpected binary operator"),
            Token::PinIndex(index) => {
                // could be the start of the binary op or just a lone PinIndex
                match tokens.get(1) {
                    Some(Token::BinaryOp(_)) => {
                        // convert to unambiguous form and try again
                        tokens.insert(1, Token::CloseBracket);
                        tokens.insert(0, Token::OpenBracket);
                        Self::munch_tokens(tokens, depth - 1)
                    }
                    Some(Token::CloseBracket) | None => {
                        // lone token
                        let pin_index = *index;
                        tokens.remove(0);
                        Ok(AstNode::PinIndex(pin_index))
                    }
                    Some(_) => Err("PinIndex followed by invalid token"),
                }
            }
        }
    }

    fn matches(&self, inputs: &Vec<bool>) -> bool {
        match self {
            Self::Negate(inverted) => !inverted.matches(inputs),
            Self::PinIndex(index) => inputs[*index as usize],
            Self::Binary(BinaryOp::And, a1, a2) => a1.matches(inputs) && a2.matches(inputs),
            Self::Binary(BinaryOp::Or, a1, a2) => a1.matches(inputs) || a2.matches(inputs),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Expr(AstNode, pub u32);

impl Expr {
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        let mut tokens: VecDeque<Token> = lex(s)?.into_iter().collect();

        let mut pin_indices: Vec<_> = tokens
            .iter()
            .filter_map(|t| match t {
                Token::PinIndex(index) => Some(index),
                _ => None,
            })
            .collect();

        pin_indices.sort();
        pin_indices.dedup();

        let input_pin_count = pin_indices.len() as u32;

        if tokens.is_empty() {
            return Err("No tokens could be parsed. Is the expression string empty?");
        }

        let root_ast_node = AstNode::munch_tokens(&mut tokens, MAX_RECURSION_DEPTH)?;
        Ok(Self(root_ast_node, input_pin_count))
    }

    pub fn evaluate(&self, inputs: &Vec<bool>) -> bool {
        self.0.matches(inputs)
    }
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

#[test]
fn test_invalid_syntax() {
    assert!(Expr::from_string("").is_err());
    assert!(Expr::from_string("!!").is_err());
    assert!(Expr::from_string(")").is_err());
    assert!(Expr::from_string("a").is_err());
    assert!(Expr::from_string("1(").is_err());
    assert!(Expr::from_string("1!").is_err());
    assert!(Expr::from_string("0 &").is_err());
    assert!(Expr::from_string("(0 & 1").is_err());
    assert!(Expr::from_string("() 0").is_err());
    assert!(Expr::from_string("0 1").is_err());
}

#[test]
fn test_simple_negation() {
    assert_eq!(
        Expr::from_string("!1"),
        Ok(Expr(AstNode::Negate(Box::new(AstNode::PinIndex(1))), 1))
    );
}

#[test]
fn test_simple_expressions() {
    assert!(Expr::from_string("!0 & (1 | 2)")
        .unwrap()
        .evaluate(&vec![false, true, false]));

    assert!(Expr::from_string("!0 & (1 | 2)")
        .unwrap()
        .evaluate(&vec![false, true, true]));

    assert!(!Expr::from_string("!0 & (1 | 2)")
        .unwrap()
        .evaluate(&vec![true, false, true]));
}

#[test]
fn test_bracketed_expressions() {
    assert!(Expr::from_string("0 | (1 & 2)")
        .unwrap()
        .evaluate(&vec![true, false, false]));

    assert!(Expr::from_string("(0 & 1) | 2")
        .unwrap()
        .evaluate(&vec![false, false, true]));

    assert!(Expr::from_string("(0 & 1) | 2)")
        .unwrap()
        .evaluate(&vec![true, true, false]));
}

#[test]
fn test_max_recursion_depth() {
    assert!(
        Expr::from_string("((((((((((((((((((((((((((((((0))))))))))))))))))))))))))))))").is_err(),
    );
    assert!(
        Expr::from_string("(((((((((((((((((((((((((((((0)))))))))))))))))))))))))))))").is_ok()
    );
}
