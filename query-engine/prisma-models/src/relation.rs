use crate::*;
use dml::ReferentialAction;
use psl::datamodel_connector::{walker_ext_traits::RelationFieldWalkerExt, RelationMode};
use std::fmt::Debug;

pub type Relation = Cursor<psl::parser_database::RelationId>;
pub type RelationRef = Relation;
pub type RelationWeakRef = Relation;

impl Relation {
    pub const MODEL_A_DEFAULT_COLUMN: &'static str = "A";
    pub const MODEL_B_DEFAULT_COLUMN: &'static str = "B";
    pub const TABLE_ALIAS: &'static str = "RelationTable";

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns `true` only if the `Relation` is just a link between two
    /// `RelationField`s.
    pub fn is_inline_relation(&self) -> bool {
        self.walker().refine().as_inline().is_some()
    }

    /// Returns `true` if the `Relation` is a table linking two models.
    pub fn is_relation_table(&self) -> bool {
        !self.is_inline_relation()
    }

    /// A model that relates to itself. For example a `Person` that is a parent
    /// can relate to people that are children.
    pub fn is_self_relation(&self) -> bool {
        self.walker().is_self_relation()
    }

    // /// A pointer to the first `Model` in the `Relation`.
    // pub fn model_a(&self) -> ModelRef {
    //     self.model_a
    //         .get_or_init(|| {
    //             let model = self.internal_data_model().find_model(&self.model_a_name).unwrap();
    //             Arc::downgrade(&model)
    //         })
    //         .upgrade()
    //         .expect("Model A deleted without deleting the relations in internal_data_model.")
    // }

    // /// A pointer to the second `Model` in the `Relation`.
    // pub fn model_b(&self) -> ModelRef {
    //     self.model_b
    //         .get_or_init(|| {
    //             let model = self.internal_data_model().find_model(&self.model_b_name).unwrap();
    //             Arc::downgrade(&model)
    //         })
    //         .upgrade()
    //         .expect("Model B deleted without deleting the relations in internal_data_model.")
    // }

    // /// A pointer to the `RelationField` in the first `Model` in the `Relation`.
    // pub fn field_a(&self) -> RelationFieldRef {
    //     self.field_a
    //         .get_or_init(|| {
    //             let field = self
    //                 .model_a()
    //                 .fields()
    //                 .find_from_relation(&self.name, RelationSide::A)
    //                 .unwrap();

    //             Arc::downgrade(&field)
    //         })
    //         .upgrade()
    //         .expect("Field A deleted without deleting the relations in internal_data_model.")
    // }

    // /// A pointer to the `RelationField` in the second `Model` in the `Relation`.
    // pub fn field_b(&self) -> RelationFieldRef {
    //     self.field_b
    //         .get_or_init(|| {
    //             let field = self
    //                 .model_b()
    //                 .fields()
    //                 .find_from_relation(&self.name, RelationSide::B)
    //                 .unwrap();

    //             Arc::downgrade(&field)
    //         })
    //         .upgrade()
    //         .expect("Field B deleted without deleting the relations in internal_data_model.")
    // }

    /// Practically deprecated with Prisma 2.
    pub fn is_many_to_many(&self) -> bool {
        self.walker().refine().as_many_to_many().is_some()
    }

    pub fn is_one_to_one(&self) -> bool {
        self.walker()
            .refine()
            .as_inline()
            .map(|r| r.is_one_to_one())
            .unwrap_or_default()
    }

    pub fn is_one_to_many(&self) -> bool {
        self.walker()
            .refine()
            .as_inline()
            .map(|r| !r.is_one_to_one())
            .unwrap_or_default()
    }

    /// Retrieves the onDelete policy for this relation.
    pub fn on_delete(&self) -> ReferentialAction {
        match self.walker().refine() {
            parser_database::walkers::RefinedRelationWalker::ImplicitManyToMany(_)
            | parser_database::walkers::RefinedRelationWalker::TwoWayEmbeddedManyToMany(_) => {
                ReferentialAction::Cascade
            }
            parser_database::walkers::RefinedRelationWalker::Inline(rel) => {
                let field = rel.forward_relation_field().unwrap();
                let action = field.explicit_on_delete().unwrap_or_else(|| {
                    field.default_on_delete_action(self.dm.schema.relation_mode(), self.dm.schema.connector)
                });
                match (action, self.dm.schema.relation_mode()) {
                    // NoAction is an alias for Restrict when relationMode = "prisma"
                    (ReferentialAction::NoAction, RelationMode::Prisma) => ReferentialAction::Restrict,
                    (action, _) => action,
                }
            }
        }
    }

    /// Retrieves the onUpdate policy for this relation.
    pub fn on_update(&self) -> ReferentialAction {
        match self.walker().refine() {
            parser_database::walkers::RefinedRelationWalker::ImplicitManyToMany(_)
            | parser_database::walkers::RefinedRelationWalker::TwoWayEmbeddedManyToMany(_) => {
                ReferentialAction::Cascade
            }
            parser_database::walkers::RefinedRelationWalker::Inline(rel) => {
                let field = rel.forward_relation_field().unwrap();
                let action = field.explicit_on_update().unwrap_or(ReferentialAction::Cascade);
                match (action, self.dm.schema.relation_mode()) {
                    // NoAction is an alias for Restrict when relationMode = "prisma"
                    (ReferentialAction::NoAction, RelationMode::Prisma) => ReferentialAction::Restrict,
                    (action, _) => action,
                }
            }
        }
    }

    pub fn manifestation(&self) -> &RelationLinkManifestation {
        &self.manifestation
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationLinkManifestation {
    Inline(InlineRelation),
    RelationTable(RelationTable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineRelation {
    pub in_table_of_model_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RelationTable {
    pub table: String,
    pub model_a_column: String,
    pub model_b_column: String,
}
