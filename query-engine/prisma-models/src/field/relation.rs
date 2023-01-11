use crate::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RelationField(Cursor<(ast::ModelId, ast::FieldId)>);

pub type RelationFieldRef = RelationField;
pub type RelationFieldWeak = RelationField;

impl RelationField {
    fn walker(&self) -> parser_database::walkers::RelationFieldWalker<'_> {
        self.0.dm.schema.db.walk(self.0.id.0).relation_field(self.0.id.1)
    }

    fn arity(&self) -> ast::FieldArity {
        self.0.dm.schema.db.ast()[self.0.id.0][self.0.id.1].arity
    }

    pub fn name(&self) -> &str {
        self.walker().name()
    }

    pub fn is_list(&self) -> bool {
        self.arity().is_list()
    }

    pub fn is_required(&self) -> bool {
        self.arity().is_required()
    }

    ///// Returns the `FieldSelection` used for this relation fields model.
    /////
    ///// ## What is the field selection of a relation field?
    /////
    ///// The set of fields required by the relation (**on the model of the relation field**) to be able to link the related records.
    /////
    ///// In case of a many-to-many relation field, we can make the assumption that the primary identifier of the enclosing model
    ///// is the set of linking fields, as this is how Prisma many-to-many works and we only support implicit join tables (i.e. m:n)
    ///// in the Prisma style.
    //pub fn linking_fields(&self) -> FieldSelection {
    //    let walker = self.walker();
    //    match walker.relation().refine() {
    //        parser_database::walkers::RefinedRelationWalker::ImplicitManyToMany(_)
    //        | parser_database::walkers::RefinedRelationWalker::TwoWayEmbeddedManyToMany(_) => {
    //            self.model().primary_identifier()
    //        }
    //        parser_database::walkers::RefinedRelationWalker::Inline(_) => {
    //            if self.relation_info.references.is_empty() {
    //                let related_field = self.related_field();
    //                let model = self.model();
    //                let fields = model.fields();

    //                let referenced_fields: Vec<_> = related_field
    //                    .relation_info
    //                    .references
    //                    .iter()
    //                    .map(|field_name| {
    //                        fields
    //                            .find_from_all(field_name)
    //                            .unwrap_or_else(|_| {
    //                                panic!(
    //                                    "Invalid data model: To field {} can't be resolved on model {}",
    //                                    field_name, model.name
    //                                )
    //                            })
    //                            .clone()
    //                    })
    //                    .collect();

    //                FieldSelection::from(referenced_fields)
    //            } else {
    //                FieldSelection::from(self)
    //            }
    //        }
    //    }
    //}

    pub fn is_optional(&self) -> bool {
        !self.is_required()
    }

    pub fn model(&self) -> Model {
        self.0.refocus(self.0.id.0)
    }

    pub fn scalar_fields(&self) -> impl Iterator<Item = ScalarField> + Clone + '_ {
        self.walker()
            .referencing_fields()
            .into_iter()
            .flatten()
            .map(|field| ScalarField(self.0.refocus((field.model().id, field.field_id()))))
    }

    pub fn relation(&self) -> Relation {
        self.0.refocus(self.walker().relation().id)
    }

    // /// Alias for more clarity (in most cases, doesn't add more clarity for self-relations);
    // pub fn is_inlined_on_enclosing_model(&self) -> bool {
    //     self.relation_is_inlined_in_parent()
    // }

    // /// Inlined in self / model of self
    // pub fn relation_is_inlined_in_parent(&self) -> bool {
    //     let relation = &self.relation();

    //     match &relation.manifestation {
    //         RelationLinkManifestation::Inline(ref m) => {
    //             let is_self_rel = relation.is_self_relation();

    //             if is_self_rel {
    //                 !self.relation_info.references.is_empty()
    //             } else {
    //                 m.in_table_of_model_name == self.model().name
    //             }
    //         }
    //         _ => false,
    //     }
    // }

    // pub fn relation_is_inlined_in_child(&self) -> bool {
    //     self.relation().is_inline_relation() && !self.relation_is_inlined_in_parent()
    // }

    pub fn related_model(&self) -> ModelRef {
        self.0.refocus(self.walker().related_model().id)
    }

    pub fn related_field(&self) -> RelationField {
        let w = self.walker();
        let relation_name = w.relation_name();
        let rf = w
            .related_model()
            .relation_fields()
            .find(|rf| rf.relation_name() == relation_name)
            .unwrap();
        RelationField(self.0.refocus((rf.model().id, rf.field_id())))
    }

    // pub fn is_relation_with_name_and_side(&self, relation_name: &str, side: RelationSide) -> bool {
    //     self.relation().name == relation_name && self.relation_side == side
    // }

    // pub fn type_identifiers_with_arities(&self) -> Vec<(TypeIdentifier, FieldArity)> {
    //     self.scalar_fields()
    //         .iter()
    //         .map(|f| f.type_identifier_with_arity())
    //         .collect()
    // }

    // pub fn referenced_fields(&self) -> Vec<ScalarFieldRef> {
    //     self.relation_info
    //         .references
    //         .iter()
    //         .map(|field_name| self.related_model().fields().find_from_scalar(field_name).unwrap())
    //         .collect()
    // }

    // // Scalar fields on the left (source) side of the relation if starting traversal from `self`.
    // // Todo This is provisionary.
    // pub fn left_scalars(&self) -> Vec<ScalarFieldRef> {
    //     if self.relation().is_many_to_many() {
    //         self.model()
    //             .primary_identifier()
    //             .as_scalar_fields()
    //             .expect("Left scalars contain non-scalar selections.")
    //     } else if self.is_inlined_on_enclosing_model() {
    //         self.scalar_fields()
    //     } else {
    //         self.related_field().referenced_fields()
    //     }
    // }

    // // Scalar fields on the right (target) side of the relation if starting traversal from `self`.
    // // Todo This is provisionary.
    // pub fn right_scalars(&self) -> Vec<ScalarFieldRef> {
    //     if self.relation().is_many_to_many() {
    //         let related_field = self.related_field();
    //         let related_model = self.related_model();

    //         if related_field.relation_info.fields.is_empty() {
    //             related_model
    //                 .primary_identifier()
    //                 .as_scalar_fields()
    //                 .expect("Right scalars contain non-scalar selections.")
    //         } else {
    //             related_field
    //                 .relation_info
    //                 .fields
    //                 .iter()
    //                 .map(|f| related_model.fields().find_from_scalar(f.as_str()).unwrap())
    //                 .collect()
    //         }
    //     } else if self.is_inlined_on_enclosing_model() {
    //         self.referenced_fields()
    //     } else {
    //         self.related_field().scalar_fields()
    //     }
    // }

    // pub fn db_names(&self) -> impl Iterator<Item = String> {
    //     self.scalar_fields().into_iter().map(|f| f.db_name().to_owned())
    // }

    // pub fn on_delete(&self) -> Option<ReferentialAction> {
    //     let walker = self.walker();
    //     let on_delete = walker.explicit_on_delete().unwrap_or_else(|| {
    //         walker.default_on_delete_action(self.0.dm.schema.relation_mode(), self.0.dm.schema.connector)
    //     });
    //     Some(on_delete)
    // }

    // pub fn on_update(&self) -> Option<ReferentialAction> {
    //     self.walker().explicit_on_update()
    // }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RelationSide {
    A,
    B,
}

impl RelationSide {
    pub fn opposite(self) -> RelationSide {
        match self {
            RelationSide::A => RelationSide::B,
            RelationSide::B => RelationSide::A,
        }
    }

    pub fn is_a(self) -> bool {
        self == RelationSide::A
    }

    pub fn is_b(self) -> bool {
        self == RelationSide::B
    }
}
