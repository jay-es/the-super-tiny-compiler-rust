use crate::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
enum Node {
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression { name: String, params: Vec<Node> },
}
#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    body: Vec<Node>,
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

fn walk(tokens: &Vec<Token>, counter: &mut Counter) -> Node {
    match &tokens[counter.count] {
        Token::Number(value) => {
            counter.increment();
            Node::NumberLiteral(value.to_string())
        }
        Token::String(value) => {
            counter.increment();
            Node::StringLiteral(value.to_string())
        }
        Token::ParenOpen => {
            counter.increment();
            if let Token::Name(value) = &tokens[counter.count] {
                let mut params: Vec<Node> = vec![];
                counter.increment();

                loop {
                    if let Token::ParenClose = &tokens[counter.count] {
                        break;
                    }

                    params.push(walk(tokens, counter));
                }

                counter.increment();

                return Node::CallExpression {
                    name: value.to_string(),
                    params,
                };
            }
            panic!();
        }
        Token::ParenClose => panic!("ParenClose"),
        Token::Name(value) => panic!("Name: {}", value),
    }
}

pub fn parser(tokens: Vec<Token>) -> Result<Ast, String> {
    let mut counter = Counter::new();
    let mut body: Vec<Node> = vec![];

    while counter.count < tokens.len() {
        body.push(walk(&tokens, &mut counter));
    }

    Ok(Ast { body })
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
        let excepted = Ast {
            body: vec![Node::CallExpression {
                name: "add".to_string(),
                params: vec![
                    Node::NumberLiteral("2".to_string()),
                    Node::CallExpression {
                        name: "subtract".to_string(),
                        params: vec![
                            Node::NumberLiteral("4".to_string()),
                            Node::NumberLiteral("2".to_string()),
                        ],
                    },
                ],
            }],
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
