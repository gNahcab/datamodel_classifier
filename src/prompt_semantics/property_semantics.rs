struct PropertySemantics {
    /// this structure will be fed to the classifiers
    /// FYI: the candidates will be chosen and filtered (because of the constraints of the ontology reference selected for the resource this property is part of) elsewhere
    property_name: String,
    label: Option<String>,
    description: Option<String>,
    comment: Option<String>,
    value_kind: ValueKind,
    examples: Vec<String>,
}

enum ValueKind {
    TextValue,
    IntValue,
    DateValue,
}