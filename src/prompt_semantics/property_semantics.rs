struct PropertySemantics {
    /// this structure will be fed to the classifiers
    property_name: String,
    label: Option<String>,
    description: Option<String>,
    value_kind: ValueKind,
    examples: Vec<String>,
}

enum ValueKind {
    TextValue,
    IntValue,
    DateValue,
}