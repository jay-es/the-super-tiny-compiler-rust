use crate::parser::{AstNode, AstNodeKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NewAstNodeKind {
    Program(Vec<NewAstNode>),
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression {
        callee_name: String,
        arguments: Vec<NewAstNode>,
    },
    ExpressionStatement {
        callee_name: String,
        arguments: Vec<NewAstNode>,
    },
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewAstNode {
    pub kind: NewAstNodeKind,
}

fn traverser(ast: &mut AstNode, _visitor: ()) {
    fn traverse_array(array: &mut Vec<AstNode>, parant: &mut AstNode) {
        for child in array {
            traverse_node(child, Some(parant));
        }
    }

    fn traverse_node(node: &mut AstNode, parent: Option<&mut AstNode>) {
        // visitor
        // let methods = visitor[node.type]
        // if methods && methods.enter {
        //     methods.enter(node, parent)
        // }
        match &node.kind {
            AstNodeKind::NumberLiteral(value) => {
                let kind = NewAstNodeKind::NumberLiteral(value.to_owned());
                parent.unwrap().context.push(NewAstNode { kind });
            }
            AstNodeKind::StringLiteral(value) => {
                let kind = NewAstNodeKind::StringLiteral(value.to_owned());
                parent.unwrap().context.push(NewAstNode { kind });
            }
            AstNodeKind::CallExpression { name, params: _ } => {
                if let AstNodeKind::CallExpression { name: _, params: _ } =
                    parent.as_ref().unwrap().kind.clone()
                {
                    let kind = NewAstNodeKind::CallExpression {
                        callee_name: name.to_owned(),
                        arguments: node.context.clone(), // TODO: clone で問題ないか。ダメなら static 参照
                    };
                    parent.unwrap().context.push(NewAstNode { kind });
                } else {
                    let kind = NewAstNodeKind::ExpressionStatement {
                        callee_name: name.to_owned(),
                        arguments: node.context.clone(), // TODO: clone で問題ないか。ダメなら static 参照
                    };
                    parent.unwrap().context.push(NewAstNode { kind });
                }
            }
            _ => (),
        }

        match &mut node.kind.clone() {
            AstNodeKind::Program(body) => traverse_array(body, node),
            AstNodeKind::CallExpression { name: _, params } => traverse_array(params, node),
            _ => (),
        }

        // if methods && methods.exit {
        //     methods.exit(node, parent)
        // }
    }

    traverse_node(ast, None)
}

pub fn transformer(mut ast: AstNode) -> Result<NewAstNode, String> {
    traverser(&mut ast, ());

    let body: Vec<NewAstNode> = ast.context;

    Ok(NewAstNode {
        kind: NewAstNodeKind::Program(body),
    })
}

#[cfg(test)]
mod transformer_test {
    use super::*;

    #[test]
    fn サンプル() {
        let input = AstNode {
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

        let excepted = NewAstNode {
            kind: NewAstNodeKind::Program(vec![NewAstNode {
                kind: NewAstNodeKind::ExpressionStatement {
                    callee_name: "add".to_string(),
                    arguments: vec![
                        NewAstNode {
                            kind: NewAstNodeKind::NumberLiteral("2".to_string()),
                        },
                        NewAstNode {
                            kind: NewAstNodeKind::CallExpression {
                                callee_name: "subtract".to_string(),
                                arguments: vec![
                                    NewAstNode {
                                        kind: NewAstNodeKind::NumberLiteral("4".to_string()),
                                    },
                                    NewAstNode {
                                        kind: NewAstNodeKind::NumberLiteral("2".to_string()),
                                    },
                                ],
                            },
                        },
                    ],
                },
            }]),
        };

        assert_eq!(transformer(input).unwrap(), excepted);
    }
}
