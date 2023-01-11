use crate::*;

pub(crate) enum ParentContainerId {
    Model(ast::ModelId),
    CompositeType(ast::CompositeTypeId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParentContainer {
    Model(Model),
    CompositeType(CompositeType),
}

impl ParentContainer {
    pub fn as_model(&self) -> Option<&Model> {
        match self {
            ParentContainer::Model(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_composite(&self) -> Option<&CompositeType> {
        match self {
            ParentContainer::CompositeType(ct) => Some(ct),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ParentContainer::Model(model) => model.name(),
            ParentContainer::CompositeType(composite) => composite.name(),
        }
    }

    // pub fn fields(&self) -> Vec<Field> {
    //     match self {
    //         ParentContainer::Model(model) => model.upgrade().unwrap().fields().all.clone(),
    //         ParentContainer::CompositeType(composite) => composite.upgrade().unwrap().fields().to_vec(),
    //     }
    // }

    // pub fn find_field(&self, prisma_name: &str) -> Option<Field> {
    //     // Unwraps are safe: This can never fail, the models and composites are always available in memory.
    //     match self {
    //         ParentContainer::Model(weak) => weak
    //             .upgrade()
    //             .unwrap()
    //             .fields()
    //             .find_from_all(prisma_name)
    //             .ok()
    //             .cloned(),

    //         ParentContainer::CompositeType(weak) => weak
    //             .upgrade()
    //             .unwrap()
    //             .fields()
    //             .iter()
    //             .find(|field| field.name() == prisma_name)
    //             .cloned(),
    //     }
    // }

    pub fn is_composite(&self) -> bool {
        matches!(self, ParentContainer::CompositeType(..))
    }

    pub fn is_model(&self) -> bool {
        matches!(self, ParentContainer::Model(..))
    }
}

impl From<Model> for ParentContainer {
    fn from(model: Model) -> Self {
        ParentContainer::Model(model)
    }
}

impl From<CompositeType> for ParentContainer {
    fn from(composite: CompositeType) -> Self {
        ParentContainer::CompositeType(composite)
    }
}
