pub struct ResourceObject {
    datamodel_name: String,
    res_name: String,
}
pub enum PropertyObject {
    Resource(ResourceObject),
    TextValue,
    DateValue,
    ListValue
}