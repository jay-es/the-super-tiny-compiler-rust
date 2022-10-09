use crate::parser::{AstNode, AstNodeKind};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NewAstNode {
    Program(Rc<RefCell<Vec<NewAstNode>>>),
    Identifier(String),
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression {
        callee: Box<NewAstNode>,
        arguments: Rc<RefCell<Vec<NewAstNode>>>,
    },
    ExpressionStatement(Box<NewAstNode>),
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
            AstNodeKind::CallExpression { params, .. } => traverse_array(params, node),
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
                let new_ast_node = NewAstNode::NumberLiteral(value.to_owned());
                parent_context.borrow_mut().push(new_ast_node);
            }
            AstNodeKind::StringLiteral(value) => {
                let new_ast_node = NewAstNode::StringLiteral(value.to_owned());
                parent_context.borrow_mut().push(new_ast_node);
            }
            AstNodeKind::CallExpression { name, .. } => {
                let new_ast_node = NewAstNode::CallExpression {
                    callee: Box::new(NewAstNode::Identifier(name.to_owned())),
                    arguments: Rc::clone(&node.context),
                };
                if let AstNodeKind::CallExpression { .. } = parent_kind {
                    parent_context.borrow_mut().push(new_ast_node);
                } else {
                    let new_ast_node = NewAstNode::ExpressionStatement(Box::new(new_ast_node));
                    parent_context.borrow_mut().push(new_ast_node);
                }
            }
            _ => (),
        }
    }
}

fn exit(_node: &AstNode, _parent: &Option<&mut AstNode>) {}

pub fn transformer(mut ast: AstNode) -> Result<NewAstNode, String> {
    traverser(&mut ast, ());

    Ok(NewAstNode::Program(ast.context.to_owned()))
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

        let excepted = NewAstNode::Program(Rc::new(RefCell::new(vec![
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

        assert_eq!(transformer(input).unwrap(), excepted);
    }
}
