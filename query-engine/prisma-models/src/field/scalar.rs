use psl::{
    datamodel_connector::{walker_ext_traits::ScalarFieldWalkerExt, NativeTypeInstance},
    dml::FieldArity,
};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScalarField(pub(crate) Cursor<(ast::ModelId, ast::FieldId)>);

pub type ScalarFieldRef = ScalarField;
pub type ScalarFieldWeak = ScalarField;

impl ScalarField {
    pub fn internal_data_model(&self) -> &crate::InternalDataModel {
        self.0.internal_data_model()
    }

    fn walker(&self) -> parser_database::walkers::ScalarFieldWalker<'_> {
        self.0.walker().walk(self.0.id.0).scalar_field(self.0.id.1)
    }

    pub fn arity(&self) -> ast::FieldArity {
        self.walker().ast_field().arity
    }

    pub fn name(&self) -> &str {
        self.walker().name()
    }

    pub fn is_id(&self) -> bool {
        self.walker().is_single_pk()
    }

    pub fn is_list(&self) -> bool {
        self.arity().is_list()
    }

    pub fn is_required(&self) -> bool {
        self.arity().is_required()
    }

    pub fn unique(&self) -> bool {
        self.walker().is_unique()
    }

    pub fn db_name(&self) -> &str {
        self.walker().database_name()
    }

    pub fn native_type_instance(&self) -> Option<NativeTypeInstance> {
        self.walker().native_type_instance(self.0.dm.schema.connector)
    }

    pub fn type_identifier(&self) -> TypeIdentifier {
        match self.walker().scalar_field_type() {
            parser_database::ScalarFieldType::CompositeType(_) => {
                todo!("composite type support in datamodel_converter")
            }
            parser_database::ScalarFieldType::Enum(id) => {
                TypeIdentifier::Enum(self.0.dm.schema.db.walk(id).name().to_owned())
            }
            parser_database::ScalarFieldType::BuiltInScalar(_) => todo!(),
            parser_database::ScalarFieldType::Unsupported(_) => TypeIdentifier::Unsupported,
        }
    }

    pub fn type_identifier_with_arity(&self) -> (TypeIdentifier, FieldArity) {
        (self.type_identifier(), self.arity())
    }

    // pub fn is_numeric(&self) -> bool {
    //     self.type_identifier.is_numeric()
    // }

    pub fn container(&self) -> ParentContainer {
        ParentContainer::Model(self.0.refocus(self.0.id.0))
    }

    pub fn internal_enum(&self) -> Option<InternalEnum> {
        self.walker().field_type_as_enum().map(|w| self.0.refocus(w.id))
    }

    pub fn default_value(&self) -> Option<&dml::DefaultValue> {
        todo!()
    }

    pub fn is_updated_at(&self) -> bool {
        self.walker().is_updated_at()
    }

    pub fn is_auto_generated_int_id(&self) -> bool {
        let w = self.walker();
        w.is_autoincrement() && w.is_single_pk()
    }

    // pub fn native_type(&self) -> Option<&NativeTypeInstance> {
    //     self.native_type.as_ref()
    // }

    pub fn is_autoincrement(&self) -> bool {
        self.walker().is_autoincrement()
    }
}
