use crate::reference_ontologies::onto_name::OntoName;

pub struct ReferenceTriple {
    onto_name: OntoName,
    subject: Option<String>,
    predicate: Option<String>,
    object: Option<String>,
}