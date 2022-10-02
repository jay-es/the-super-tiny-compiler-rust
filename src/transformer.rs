use crate::parser::{AstNode, AstNodeKind};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NewAstNodeKind {
    Program(Rc<RefCell<Vec<NewAstNode>>>),
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression {
        callee_name: String,
        arguments: Rc<RefCell<Vec<NewAstNode>>>,
    },
    ExpressionStatement {
        callee_name: String,
        arguments: Rc<RefCell<Vec<NewAstNode>>>,
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
        enter(node, &parent);

        match &mut node.kind.clone() {
            AstNodeKind::Program(body) => traverse_array(body, node),
            AstNodeKind::CallExpression { name: _, params } => traverse_array(params, node),
            _ => (),
        }

        exit(node, &parent);
    }

    traverse_node(ast, None)
}

fn enter(node: &AstNode, parent: &Option<&mut AstNode>) {
    if let Some(AstNode {
        context: parent_context,
        kind: parent_kind,
    }) = parent
    {
        match &node.kind {
            AstNodeKind::NumberLiteral(value) => {
                let kind = NewAstNodeKind::NumberLiteral(value.to_owned());
                parent_context.borrow_mut().push(NewAstNode { kind });
            }
            AstNodeKind::StringLiteral(value) => {
                let kind = NewAstNodeKind::StringLiteral(value.to_owned());
                parent_context.borrow_mut().push(NewAstNode { kind });
            }
            AstNodeKind::CallExpression { name, params: _ } => {
                if let AstNodeKind::CallExpression { name: _, params: _ } = parent_kind {
                    let kind = NewAstNodeKind::CallExpression {
                        callee_name: name.to_owned(),
                        arguments: Rc::clone(&node.context),
                    };
                    parent_context.borrow_mut().push(NewAstNode { kind });
                } else {
                    let kind = NewAstNodeKind::ExpressionStatement {
                        callee_name: name.to_owned(),
                        arguments: Rc::clone(&node.context),
                    };
                    parent_context.borrow_mut().push(NewAstNode { kind });
                }
            }
            _ => (),
        }
    }
}

fn exit(_node: &AstNode, _parent: &Option<&mut AstNode>) {}

pub fn transformer(mut ast: AstNode) -> Result<NewAstNode, String> {
    traverser(&mut ast, ());

    Ok(NewAstNode {
        kind: NewAstNodeKind::Program(ast.context.to_owned()),
    })
}

#[cfg(test)]
mod transformer_test {
    use super::*;

    #[test]
    fn サンプル() {
        let input = AstNode {
            context: Rc::new(RefCell::new(vec![])),
            kind: AstNodeKind::Program(vec![AstNode {
                context: Rc::new(RefCell::new(vec![])),
                kind: AstNodeKind::CallExpression {
                    name: "add".to_string(),
                    params: vec![
                        AstNode {
                            context: Rc::new(RefCell::new(vec![])),
                            kind: AstNodeKind::NumberLiteral("2".to_string()),
                        },
                        AstNode {
                            context: Rc::new(RefCell::new(vec![])),
                            kind: AstNodeKind::CallExpression {
                                name: "subtract".to_string(),
                                params: vec![
                                    AstNode {
                                        context: Rc::new(RefCell::new(vec![])),
                                        kind: AstNodeKind::NumberLiteral("4".to_string()),
                                    },
                                    AstNode {
                                        context: Rc::new(RefCell::new(vec![])),
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
            kind: NewAstNodeKind::Program(Rc::new(RefCell::new(vec![NewAstNode {
                kind: NewAstNodeKind::ExpressionStatement {
                    callee_name: "add".to_string(),
                    arguments: Rc::new(RefCell::new(vec![
                        NewAstNode {
                            kind: NewAstNodeKind::NumberLiteral("2".to_string()),
                        },
                        NewAstNode {
                            kind: NewAstNodeKind::CallExpression {
                                callee_name: "subtract".to_string(),
                                arguments: Rc::new(RefCell::new(vec![
                                    NewAstNode {
                                        kind: NewAstNodeKind::NumberLiteral("4".to_string()),
                                    },
                                    NewAstNode {
                                        kind: NewAstNodeKind::NumberLiteral("2".to_string()),
                                    },
                                ])),
                            },
                        },
                    ])),
                },
            }]))),
        };

        assert_eq!(transformer(input).unwrap(), excepted);
    }
}
