use std::fmt::Debug;

use super::{Env, ExpressionResult, InterpretationResult};
use crate::Query;

pub enum Expression {
    Sequence {
        seq: Vec<Expression>,
    },

    Func {
        func: Box<dyn FnOnce(Env) -> InterpretationResult<Expression> + Send + Sync + 'static>,
    },

    Query {
        query: Box<Query>,
    },

    Let {
        bindings: Vec<Binding>,
        expressions: Vec<Expression>,
    },

    Get {
        binding_name: String,
    },

    GetFirstNonEmpty {
        binding_names: Vec<String>,
    },

    If {
        func: Box<dyn FnOnce() -> bool + Send + Sync + 'static>,
        then: Vec<Expression>,
        else_: Vec<Expression>,
    },

    Return {
        result: Box<ExpressionResult>,
    },
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Sequence { seq } => seq.fmt(f),
            Expression::Func { .. } => write!(f, "Func"),
            Expression::Query { query } => query.fmt(f),
            Expression::Let { bindings, expressions } => f
                .debug_struct("Let")
                .field("bindings", bindings)
                .field("expressions", expressions)
                .finish(),
            Expression::Get { binding_name } => f.debug_struct("Get").field("binding_name", binding_name).finish(),
            Expression::GetFirstNonEmpty { binding_names } => f
                .debug_struct("GetFirstNonEmpty")
                .field("binding_names", binding_names)
                .finish(),
            Expression::If { then, else_, .. } => {
                f.debug_struct("If").field("then", then).field("else", else_).finish()
            }
            Expression::Return { result } => f.debug_struct("Return").field("result", result).finish(),
        }
    }
}

#[derive(Debug)]
pub struct Binding {
    pub name: String,
    pub expr: Expression,
}
