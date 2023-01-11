use crate::*;

pub type CompositeField = crate::Cursor<(ast::CompositeTypeId, ast::FieldId)>;
pub type CompositeFieldRef = CompositeField;
pub type CompositeFieldWeak = CompositeField;

impl CompositeField {
    pub fn name(&self) -> &str {
        self.walker().name()
    }

    pub fn is_list(&self) -> bool {
        self.walker().arity().is_list()
    }

    pub fn is_required(&self) -> bool {
        self.walker().arity().is_required()
    }

    pub fn is_optional(&self) -> bool {
        self.walker().arity().is_optional()
    }

    pub fn db_name(&self) -> &str {
        self.walker().database_name()
    }

    pub fn composite_type(&self) -> CompositeType {
        self.refocus(self.id.0)
    }

    pub fn container(&self) -> ParentContainer {
        ParentContainer::CompositeType(self.composite_type())
    }
}
