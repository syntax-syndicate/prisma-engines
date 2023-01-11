mod composite;
mod relation;
mod scalar;

pub use composite::*;
pub use relation::*;
pub use scalar::*;

use crate::*;
use psl::dml::ScalarType;

#[derive(Clone, Debug)]
pub enum Field {
    Scalar(ScalarField),
    Relation(RelationField),
    Composite(CompositeField),
}

pub type FieldWeak = Field;

impl Field {
    fn parent(&self) -> ParentContainer {
        match self {
            Field::Scalar(s) => s.container(),
            Field::Relation(r) => ParentContainer::Model(r.model()),
            Field::Composite(c) => c.container(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Field::Scalar(s) => s.name(),
            Field::Relation(r) => r.name(),
            Field::Composite(c) => c.name(),
        }
    }

    pub fn db_name(&self) -> &str {
        match self {
            Field::Scalar(ref sf) => sf.db_name(),
            Field::Relation(ref rf) => rf.name(),
            Field::Composite(ref cf) => cf.db_name(),
        }
    }

    pub fn is_scalar(&self) -> bool {
        matches!(self, Self::Scalar(_))
    }

    pub fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(..))
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, Self::Composite(_))
    }

    pub fn into_scalar(self) -> Option<ScalarFieldRef> {
        match self {
            Field::Scalar(sf) => Some(sf),
            _ => None,
        }
    }

    pub fn is_id(&self) -> bool {
        match self {
            Field::Scalar(sf) => sf.is_id(),
            Field::Relation(_) => false,
            Field::Composite(_) => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Field::Scalar(ref sf) => sf.is_list(),
            Field::Relation(ref rf) => rf.is_list(),
            Field::Composite(ref cf) => cf.is_list(),
        }
    }

    pub fn try_into_scalar(self) -> Option<ScalarFieldRef> {
        match self {
            Field::Scalar(scalar) => Some(scalar),
            _ => None,
        }
    }

    pub fn is_required(&self) -> bool {
        match self {
            Field::Scalar(ref sf) => sf.is_required(),
            Field::Relation(ref rf) => rf.is_required(),
            Field::Composite(ref cf) => cf.is_required(),
        }
    }

    pub fn is_unique(&self) -> bool {
        match self {
            Field::Scalar(ref sf) => sf.unique(),
            Field::Relation(_) => false,
            Field::Composite(_) => false,
        }
    }

    pub fn model(&self) -> Option<Model> {
        match self {
            Self::Scalar(sf) => sf.container().as_model().cloned(),
            Self::Relation(rf) => Some(rf.model()),
            Self::Composite(cf) => cf.container().as_model().cloned(),
        }
    }

    // pub fn scalar_fields(&self) -> Vec<ScalarFieldRef> {
    //     match self {
    //         Self::Scalar(sf) => vec![sf.clone()],
    //         Self::Relation(rf) => rf.scalar_fields(),
    //         Self::Composite(_cf) => vec![], // [Composites] todo
    //     }
    // }

    pub fn as_composite(&self) -> Option<&CompositeFieldRef> {
        if let Self::Composite(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_scalar(&self) -> Option<&ScalarFieldRef> {
        if let Self::Scalar(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum TypeIdentifier {
    String,
    Int,
    BigInt,
    Float,
    Decimal,
    Boolean,
    Enum(String),
    UUID,
    Json,
    Xml,
    DateTime,
    Bytes,
    Unsupported,
}

impl TypeIdentifier {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            TypeIdentifier::Int | TypeIdentifier::BigInt | TypeIdentifier::Float | TypeIdentifier::Decimal
        )
    }
}

impl std::fmt::Display for TypeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeIdentifier::String => write!(f, "String"),
            TypeIdentifier::Int => write!(f, "Int"),
            TypeIdentifier::BigInt => write!(f, "BigInt"),
            TypeIdentifier::Float => write!(f, "Float"),
            TypeIdentifier::Decimal => write!(f, "Decimal"),
            TypeIdentifier::Boolean => write!(f, "Bool"),
            TypeIdentifier::Enum(e) => write!(f, "Enum{}", e),
            TypeIdentifier::UUID => write!(f, "UUID"),
            TypeIdentifier::Json => write!(f, "Json"),
            TypeIdentifier::Xml => write!(f, "Xml"),
            TypeIdentifier::DateTime => write!(f, "DateTime"),
            TypeIdentifier::Bytes => write!(f, "Bytes"),
            TypeIdentifier::Unsupported => write!(f, "Unsupported"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DateType {
    Date,
    Time,
    DateTime,
}

impl From<ScalarFieldRef> for Field {
    fn from(sf: ScalarFieldRef) -> Self {
        Field::Scalar(sf)
    }
}

impl From<RelationFieldRef> for Field {
    fn from(rf: RelationFieldRef) -> Self {
        Field::Relation(rf)
    }
}

impl From<CompositeFieldRef> for Field {
    fn from(cf: CompositeFieldRef) -> Self {
        Field::Composite(cf)
    }
}

impl From<ScalarType> for TypeIdentifier {
    fn from(st: ScalarType) -> Self {
        match st {
            ScalarType::String => Self::String,
            ScalarType::Int => Self::Int,
            ScalarType::BigInt => Self::BigInt,
            ScalarType::Float => Self::Float,
            ScalarType::Boolean => Self::Boolean,
            ScalarType::DateTime => Self::DateTime,
            ScalarType::Json => Self::Json,
            ScalarType::Decimal => Self::Decimal,
            ScalarType::Bytes => Self::Bytes,
        }
    }
}
