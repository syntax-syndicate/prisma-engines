use crate::*;

pub type CompositeType = Cursor<ast::CompositeTypeId>;
pub type CompositeTypeRef = CompositeType;
pub type CompositeTypeWeakRef = CompositeType;

impl CompositeType {
    pub fn name(&self) -> &str {
        self.walker().name()
    }

    pub fn fields(&self) -> impl ExactSizeIterator<Item = Field> + Clone + '_ {
        self.walker().fields().map(|f| Field::Composite(self.refocus(f.id)))
    }

    // pub fn find_field(&self, prisma_name: &str) -> Option<&Field> {
    //     self.fields().iter().find(|f| f.name() == prisma_name)
    // }

    pub fn find_field_by_db_name(&self, db_name: &str) -> Option<Field> {
        self.fields().find(|f| f.db_name() == db_name)
    }
}
