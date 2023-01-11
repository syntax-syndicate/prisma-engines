use crate::{CompositeType, Cursor, InternalEnumRef, Model, Relation};
use std::sync::Arc;

pub type InternalDataModelRef = InternalDataModel;
pub type InternalDataModelWeakRef = InternalDataModel;

#[derive(Debug, Clone)]
pub struct InternalDataModel {
    pub schema: Arc<psl::ValidatedSchema>,
}

impl InternalDataModel {
    pub(crate) fn focus<I>(&self, id: I) -> Cursor<I> {
        Cursor { id, dm: self.clone() }
    }

    pub fn models(&self) -> impl Iterator<Item = Model> + '_ {
        self.schema.db.walk_models().map(|m| self.focus(m.id))
    }

    pub fn composite_types(&self) -> impl Iterator<Item = CompositeType> + '_ {
        self.schema.db.walk_composite_types().map(|ct| self.focus(ct.id))
    }

    pub fn relations(&self) -> impl ExactSizeIterator<Item = Relation> + Clone + '_ {
        self.schema.db.walk_relations().map(|w| self.focus(w.id))
    }

    pub fn find_enum(&self, name: &str) -> Option<InternalEnumRef> {
        self.schema.db.find_enum(name).map(|enm| self.focus(enm.id))
    }

    // pub fn find_model(&self, name: &str) -> Option<Model> {
    //     self.models
    //         .get()
    //         .and_then(|models| models.iter().find(|model| model.name == name))
    //         .cloned()
    //         .ok_or_else(|| DomainError::ModelNotFound { name: name.to_string() })
    // }

    // /// This method takes the two models at the ends of the relation as a first argument, because
    // /// relation names are scoped by the pair of models in the relation. Relation names are _not_
    // /// globally unique.
    // pub fn find_relation(&self, model_names: (&str, &str), relation_name: &str) -> crate::Result<RelationWeakRef> {
    //     self.relations
    //         .get()
    //         .and_then(|relations| {
    //             relations
    //                 .iter()
    //                 .find(|relation| relation_matches(relation, model_names, relation_name))
    //         })
    //         .map(Arc::downgrade)
    //         .ok_or_else(|| DomainError::RelationNotFound {
    //             name: relation_name.to_owned(),
    //         })
    // }

    // /// Finds all non-list relation fields pointing to the given model.
    // /// `required` may narrow down the returned fields to required fields only. Returns all on `false`.
    // pub fn fields_pointing_to_model(&self, model: &ModelRef, required: bool) -> Vec<RelationFieldRef> {
    //     self.relation_fields()
    //         .iter()
    //         .filter(|rf| &rf.related_model() == model) // All relation fields pointing to `model`.
    //         .filter(|rf| rf.is_inlined_on_enclosing_model()) // Not a list, not a virtual field.
    //         .filter(|rf| !required || rf.is_required()) // If only required fields should be returned
    //         .map(Arc::clone)
    //         .collect()
    // }

    // /// Finds all relation fields where the foreign key refers to the given field (as either singular or compound).
    // pub fn fields_refering_to_field(&self, field: &ScalarFieldRef) -> Vec<RelationFieldRef> {
    //     match &field.container {
    //         ParentContainer::Model(model) => {
    //             let model_name = &model.upgrade().unwrap().name;

    //             self.relation_fields()
    //                 .iter()
    //                 .filter(|rf| &rf.relation_info.referenced_model == model_name)
    //                 .filter(|rf| rf.relation_info.references.contains(&field.name))
    //                 .map(Arc::clone)
    //                 .collect()
    //         }
    //         // Relation fields can not refer to composite fields.
    //         ParentContainer::CompositeType(_) => vec![],
    //     }
    // }

    // pub fn relation_fields(&self) -> &[RelationFieldRef] {
    //     self.relation_fields
    //         .get_or_init(|| {
    //             self.models()
    //                 .iter()
    //                 .flat_map(|model| model.fields().relation())
    //                 .collect()
    //         })
    //         .as_slice()
    // }
}

///// A relation's "primary key" in a Prisma schema is the relation name (defaulting to an empty
///// string) qualified by the one or two models involved in the relation.
/////
///// In other words, the scope for a relation name is only between two models. Every pair of models
///// has its own scope for relation names.
//fn relation_matches(relation: &Relation, model_names: (&str, &str), relation_name: &str) -> bool {
//    if relation.name != relation_name {
//        return false;
//    }

//    if relation.is_self_relation() && model_names.0 == model_names.1 && relation.model_a_name == model_names.0 {
//        return true;
//    }

//    if model_names.0 == relation.model_a_name && model_names.1 == relation.model_b_name {
//        return true;
//    }

//    if model_names.0 == relation.model_b_name && model_names.1 == relation.model_a_name {
//        return true;
//    }

//    false
//}
