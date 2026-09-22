use crate::reference_ontologies::datamodel_triple::datamodel_triple::DatamodelTriple;
use crate::reference_ontologies::reference_triple::reference_triple::ReferenceTriple;

struct DoubleTriple {
    dm_triple: DatamodelTriple,
    reference_triples: Vec<ReferenceTriple>
}