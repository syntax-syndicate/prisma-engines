/// Input/output type
pub enum Type {
    /// Int32
    Int32,
    /// String
    String,
}
/// query introspection result
pub struct IntrospectedQuery {
    /// raw sql text
    pub sql: String,
    /// types of the arguments
    pub arg_types: Vec<Type>,
    /// names and types of the results
    pub result_types: Vec<(String, Type)>,
}
