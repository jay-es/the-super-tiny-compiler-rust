use crate::tokenizer::Token;
use crate::transformer::NewAstNode;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNodeKind {
    Program(Vec<AstNode>),
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression { name: String, params: Vec<AstNode> },
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AstNode {
    pub context: Vec<NewAstNode>,
    pub kind: AstNodeKind,
}

struct Counter {
    count: usize,
}
impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn walk(tokens: &Vec<Token>, counter: &mut Counter) -> AstNode {
    match &tokens[counter.count] {
        Token::Number(value) => {
            counter.increment();
            AstNode {
                context: vec![],
                kind: AstNodeKind::NumberLiteral(value.to_owned()),
            }
        }
        Token::String(value) => {
            counter.increment();
            AstNode {
                context: vec![],
                kind: AstNodeKind::StringLiteral(value.to_owned()),
            }
        }
        Token::ParenOpen => {
            counter.increment();
            if let Token::Name(value) = &tokens[counter.count] {
                let mut params: Vec<AstNode> = vec![];
                counter.increment();

                loop {
                    if let Token::ParenClose = &tokens[counter.count] {
                        break;
                    }

                    params.push(walk(tokens, counter));
                }

                counter.increment();

                return AstNode {
                    context: vec![],
                    kind: AstNodeKind::CallExpression {
                        name: value.to_owned(),
                        params,
                    },
                };
            }
            panic!();
        }
        Token::ParenClose => panic!("ParenClose"),
        Token::Name(value) => panic!("Name: {}", value),
    }
}

pub fn parser(tokens: Vec<Token>) -> Result<AstNode, String> {
    let mut counter = Counter::new();
    let mut body: Vec<AstNode> = vec![];

    while counter.count < tokens.len() {
        body.push(walk(&tokens, &mut counter));
    }

    Ok(AstNode {
        context: vec![],
        kind: AstNodeKind::Program(body),
    })
}

#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn サンプル() {
        let input = vec![
            Token::ParenOpen,
            Token::Name("add".to_string()),
            Token::Number("2".to_string()),
            Token::ParenOpen,
            Token::Name("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::ParenClose,
            Token::ParenClose,
        ];
        let excepted = AstNode {
            context: vec![],
            kind: AstNodeKind::Program(vec![AstNode {
                context: vec![],
                kind: AstNodeKind::CallExpression {
                    name: "add".to_string(),
                    params: vec![
                        AstNode {
                            context: vec![],
                            kind: AstNodeKind::NumberLiteral("2".to_string()),
                        },
                        AstNode {
                            context: vec![],
                            kind: AstNodeKind::CallExpression {
                                name: "subtract".to_string(),
                                params: vec![
                                    AstNode {
                                        context: vec![],
                                        kind: AstNodeKind::NumberLiteral("4".to_string()),
                                    },
                                    AstNode {
                                        context: vec![],
                                        kind: AstNodeKind::NumberLiteral("2".to_string()),
                                    },
                                ],
                            },
                        },
                    ],
                },
            }]),
        };

        assert_eq!(parser(input).unwrap(), excepted);
    }

    #[test]
    #[should_panic]
    fn エラー() {
        let input = vec![Token::ParenClose];

        parser(input).unwrap();
    }
}
