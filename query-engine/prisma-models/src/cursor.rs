use crate::*;

#[derive(Clone)]
pub struct Cursor<I> {
    pub id: I,
    pub dm: InternalDataModel,
}

impl<I: Copy> Cursor<I> {
    pub fn internal_data_model(&self) -> &InternalDataModel {
        &self.dm
    }

    pub(crate) fn refocus<J>(&self, id: J) -> Cursor<J> {
        Cursor {
            id,
            dm: self.dm.clone(),
        }
    }

    pub fn walker(&self) -> parser_database::walkers::Walker<'_, I> {
        self.dm.schema.db.walk(self.id)
    }
}

impl<I: std::fmt::Debug> std::fmt::Debug for Cursor<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Cursor").field(&self.id).finish()
    }
}

impl<I: PartialEq> PartialEq for Cursor<I> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<I: Eq> Eq for Cursor<I> {}

impl<I: std::hash::Hash> std::hash::Hash for Cursor<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
