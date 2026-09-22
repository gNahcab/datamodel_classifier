mod reference_ontologies;
mod classify;
mod prompt_semantics;
mod datamodel;

fn main() {
    // load datamodel
    

    // read context(e.g. keywords, description etc) properties, resources
    // build a 2-tuple for properties, with the property and the expected co-domain/range

    // * Resource
    // decide which reference-ontology is relevant due to topic (if not clear or if this does not exist, just take default)
    // each type of resource (e.g. Resource, Representation, AudioRepresentation etc.) allows only a certain subset of values for each reference-ontology (this is 'hardcoded')
    // decide which value of the subset of values matches best by using the context (1), the name, label (2) and examples (3)
    //  -- loop: do so for each relevant reference ontology
    // TODO how to classify

    // * Property
    // we build now 'double-triples': they contain the resource-property-object (of the datamodel) and ALL reference-triples (the Classified-Resource(Subject):MAYBE - Property(Predicate):NONE-Object:NONE)
    // reference-triples can be NONE-NONE-NONE or (if classification was successful mapping an element from a reference-ontology to a resource) SOME-NONE-NONE
    // for every resource and for every property this resource contains we build a datamodel-triple: resource-property-object, here we can take  the 2-tuple-properties we already have built
    
    // now we have the double triple and its reference-triples (ordered by the reference-ontology), based on that we can now classify the properties each by the property (so here we would have to use a bit a more complex classification)
    // TODO how to classify
    
    // now assuming we classified correctly, we return the datamodel with the new annotations/mappings
    
    





}