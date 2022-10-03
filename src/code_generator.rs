use crate::transformer::NewAstNode;

pub fn code_generator(node: NewAstNode) -> String {
    match node {
        NewAstNode::Program(body) => body
            .borrow()
            .clone()
            .into_iter()
            .map(code_generator)
            .collect::<Vec<String>>()
            .join("\n"),
        NewAstNode::ExpressionStatement(expression) => {
            let mut str = code_generator(*expression);
            str.push(';');
            str
        }
        NewAstNode::CallExpression { callee, arguments } => {
            let args = arguments
                .borrow()
                .clone()
                .into_iter()
                .map(code_generator)
                .collect::<Vec<String>>()
                .join(", ");

            vec![
                code_generator(*callee),
                "(".to_string(),
                args,
                ")".to_string(),
            ]
            .join("")
        }
        NewAstNode::Identifier(value) => value,
        NewAstNode::NumberLiteral(value) => value,
        NewAstNode::StringLiteral(value) => {
            vec!["\"".to_string(), value, "\"".to_string()].join("")
        }
    }
}

#[cfg(test)]
mod code_generator_test {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn サンプル() {
        let input = NewAstNode::Program(Rc::new(RefCell::new(vec![
            NewAstNode::ExpressionStatement(Box::new(NewAstNode::CallExpression {
                callee: Box::new(NewAstNode::Identifier("add".to_string())),
                arguments: Rc::new(RefCell::new(vec![
                    NewAstNode::NumberLiteral("2".to_string()),
                    NewAstNode::CallExpression {
                        callee: Box::new(NewAstNode::Identifier("subtract".to_string())),
                        arguments: Rc::new(RefCell::new(vec![
                            NewAstNode::NumberLiteral("4".to_string()),
                            NewAstNode::NumberLiteral("2".to_string()),
                        ])),
                    },
                ])),
            })),
        ])));

        let excepted = "add(2, subtract(4, 2));".to_string();

        assert_eq!(code_generator(input), excepted);
    }
}
