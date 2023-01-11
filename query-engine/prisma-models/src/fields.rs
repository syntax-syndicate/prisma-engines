use crate::*;

/// Fields of a model.
pub type Fields = Model;

impl Fields {
    // pub fn id(&self) -> Option<&PrimaryKey> {
    //     self.primary_key.as_ref()
    // }

    // pub fn compound_id(&self) -> Option<&PrimaryKey> {
    //     if self
    //         .primary_key
    //         .as_ref()
    //         .map(|pk| pk.fields().len() > 1)
    //         .unwrap_or(false)
    //     {
    //         self.primary_key.as_ref()
    //     } else {
    //         None
    //     }
    // }

    // pub fn updated_at(&self) -> &Vec<ScalarFieldRef> {
    //     self.updated_at.get_or_init(|| {
    //         self.scalar_weak()
    //             .iter()
    //             .map(|sf| sf.upgrade().unwrap())
    //             .filter(|sf| sf.is_updated_at)
    //             .collect()
    //     })
    // }

    pub fn scalar_fields(&self) -> impl Iterator<Item = ScalarField> + '_ {
        self.walker()
            .scalar_fields()
            .map(|sf| ScalarField(self.refocus((sf.model().id, sf.field_id()))))
    }

    // pub fn scalar_writable(&self) -> impl Iterator<Item = ScalarFieldRef> {
    //     self.scalar().into_iter().filter(|sf| !sf.is_read_only())
    // }

    // pub fn scalar_list(&self) -> Vec<ScalarFieldRef> {
    //     self.scalar().into_iter().filter(|sf| sf.is_list()).collect()
    // }

    // pub fn relation(&self) -> Vec<RelationFieldRef> {
    //     self.relation_weak().iter().map(|f| f.upgrade().unwrap()).collect()
    // }

    // fn relation_weak(&self) -> &[RelationFieldWeak] {
    //     self.relation
    //         .get_or_init(|| self.all.iter().fold(Vec::new(), Self::relation_filter))
    //         .as_slice()
    // }

    // pub fn composite(&self) -> Vec<CompositeFieldRef> {
    //     self.composite_weak().iter().map(|f| f.upgrade().unwrap()).collect()
    // }

    // fn composite_weak(&self) -> &[CompositeFieldWeak] {
    //     self.composite
    //         .get_or_init(|| self.all.iter().fold(Vec::new(), Self::composite_filter))
    //         .as_slice()
    // }

    // pub fn non_relational(&self) -> Vec<Field> {
    //     self.scalar()
    //         .into_iter()
    //         .map(Field::from)
    //         .chain(self.composite().into_iter().map(Field::from))
    //         .collect()
    // }

    // pub fn find_many_from_all(&self, names: &BTreeSet<String>) -> Vec<&Field> {
    //     self.all.iter().filter(|field| names.contains(field.name())).collect()
    // }

    // pub fn find_many_from_scalar(&self, names: &BTreeSet<String>) -> Vec<ScalarFieldRef> {
    //     self.scalar_weak()
    //         .iter()
    //         .filter(|field| names.contains(&field.upgrade().unwrap().name))
    //         .map(|field| field.upgrade().unwrap())
    //         .collect()
    // }

    // pub fn find_many_from_relation(&self, names: &BTreeSet<String>) -> Vec<RelationFieldRef> {
    //     self.relation_weak()
    //         .iter()
    //         .filter(|field| names.contains(&field.upgrade().unwrap().name))
    //         .map(|field| field.upgrade().unwrap())
    //         .collect()
    // }

    // pub fn find_from_all(&self, prisma_name: &str) -> crate::Result<&Field> {
    //     self.all
    //         .iter()
    //         .find(|field| field.name() == prisma_name)
    //         .ok_or_else(|| DomainError::FieldNotFound {
    //             name: prisma_name.to_string(),
    //             container_name: self.model().name.clone(),
    //             container_type: "model",
    //         })
    // }

    // /// Non-virtual: Fields actually existing on the database level, this (currently) excludes relations, which are
    // /// purely virtual on a model.
    // pub fn find_from_non_virtual_by_db_name(&self, db_name: &str) -> crate::Result<&Field> {
    //     self.all
    //         .iter()
    //         .find(|field| match field {
    //             Field::Relation(_) => false,
    //             field => field.db_name() == db_name,
    //         })
    //         .ok_or_else(|| DomainError::FieldNotFound {
    //             name: db_name.to_string(),
    //             container_name: self.model().name.clone(),
    //             container_type: "model",
    //         })
    // }

    // pub fn find_from_scalar(&self, name: &str) -> crate::Result<ScalarFieldRef> {
    //     self.scalar_weak()
    //         .iter()
    //         .map(|field| field.upgrade().unwrap())
    //         .find(|field| field.name == name)
    //         .ok_or_else(|| DomainError::ScalarFieldNotFound {
    //             name: name.to_string(),
    //             container_name: self.model().name.clone(),
    //             container_type: "model",
    //         })
    // }

    // fn model(&self) -> ModelRef {
    //     self.model.upgrade().unwrap()
    // }

    // pub fn find_from_relation_fields(&self, name: &str) -> crate::Result<RelationFieldRef> {
    //     self.relation_weak()
    //         .iter()
    //         .map(|field| field.upgrade().unwrap())
    //         .find(|field| field.name == name)
    //         .ok_or_else(|| DomainError::RelationFieldNotFound {
    //             name: name.to_string(),
    //             model: self.model().name.clone(),
    //         })
    // }

    // pub fn find_from_relation(&self, name: &str, side: RelationSide) -> crate::Result<RelationFieldRef> {
    //     self.relation_weak()
    //         .iter()
    //         .map(|field| field.upgrade().unwrap())
    //         .find(|field| field.relation().name == name && field.relation_side == side)
    //         .ok_or_else(|| DomainError::FieldForRelationNotFound {
    //             relation: name.to_string(),
    //             model: self.model().name.clone(),
    //         })
    // }

    // fn scalar_filter(mut acc: Vec<ScalarFieldWeak>, field: &Field) -> Vec<ScalarFieldWeak> {
    //     if let Field::Scalar(scalar_field) = field {
    //         acc.push(Arc::downgrade(scalar_field));
    //     };

    //     acc
    // }

    // fn relation_filter(mut acc: Vec<RelationFieldWeak>, field: &Field) -> Vec<RelationFieldWeak> {
    //     if let Field::Relation(relation_field) = field {
    //         acc.push(Arc::downgrade(relation_field));
    //     };

    //     acc
    // }

    // fn composite_filter(mut acc: Vec<CompositeFieldWeak>, field: &Field) -> Vec<CompositeFieldWeak> {
    //     if let Field::Composite(composite_field) = field {
    //         acc.push(Arc::downgrade(composite_field));
    //     };

    //     acc
    // }

    // pub fn db_names(&self) -> impl Iterator<Item = String> + '_ {
    //     self.all
    //         .iter()
    //         .flat_map(|field| field.scalar_fields().into_iter().map(|f| f.db_name().to_owned()))
    //         .unique()
    // }

    // pub fn filter_all<P>(&self, predicate: P) -> Vec<Field>
    // where
    //     P: FnMut(&&Field) -> bool,
    // {
    //     self.all.iter().filter(predicate).map(Clone::clone).collect()
    // }
}
