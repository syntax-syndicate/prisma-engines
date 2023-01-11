use crate::{ast, Cursor};

pub type InternalEnum = Cursor<ast::EnumId>;
pub type InternalEnumValue = Cursor<(ast::EnumId, usize)>;
pub type InternalEnumRef = InternalEnum;
