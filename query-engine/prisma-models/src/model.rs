use crate::*;

pub type Model = Cursor<ast::ModelId>;
pub type ModelRef = Model;
pub type ModelWeakRef = Model;

impl Model {
    pub fn name(&self) -> &str {
        self.walker().name()
    }

    /// Returns the schema name for the model
    /// which is the contents of the @@schema("...") attribute
    pub fn schema_name(&self) -> Option<&str> {
        self.walker().schema_name()
    }

    /// Returns the set of fields to be used as the primary identifier for a record of that model.
    /// The identifier is nothing but an internal convention to have an anchor point for querying, or in other words,
    /// the identifier is not to be mistaken for a stable, external identifier, but has to be understood as
    /// implementation detail that is used to reason over a fixed set of fields.
    pub fn primary_identifier(&self) -> FieldSelection {
        let first_unique_criterion = self
            .walker()
            .unique_criterias()
            .find(|criterion| criterion.is_strict_criteria())
            .unwrap();
        let fields: Vec<Field> = first_unique_criterion
            .fields()
            .map(|w| match w {
                parser_database::walkers::IndexFieldWalker::Scalar(sf) => {
                    ScalarField(self.refocus((sf.model().id, sf.field_id()))).into()
                }
                parser_database::walkers::IndexFieldWalker::Composite(c) => self.refocus(c.id).into(),
            })
            .collect();
        FieldSelection::from(fields)
    }

    pub fn fields(&self) -> &Fields {
        self
    }

    pub fn supports_create_operation(&self) -> bool {
        !self
            .walker()
            .scalar_fields()
            .any(|sf| sf.is_unsupported() && sf.ast_field().arity.is_required() && sf.default_value().is_none())
    }

    /// The name of the model in the database
    /// For a sql database this will be the Table name for this model
    pub fn db_name(&self) -> &str {
        self.walker().database_name()
    }

    pub fn db_name_opt(&self) -> Option<&str> {
        self.walker().mapped_name()
    }
}
