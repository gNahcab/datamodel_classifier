# Datamodel Classifier

* why: Mapping reference ontologies consistently across multiple projects is challenging and achieving complete coherence is difficult.
* what: Combine hardcoded mapping rules with a semantic classifier and a llm to guide the consistent mapping of project data models to reference ontologies.
* how: This pipeline takes a data model and uses hardcoded rules to determine which subset of a reference ontology is applicable based on the qualities/type of Property or Resource in the data model. This set of candidate values is then passed to a small local classifier and a small local llm, together with semantic information about the Property/Resource and the project context, to determine the most appropriate mapping.

## requisites 

- brew install llama.cpp (necessary for local models running via llama.cpp on Mac M1, see scripts)

## technical details

used model for narrowing down candidate values: nomic-embed-text-v1.5-GGUF
used model for final decision on narrowed candidate values: qwen3

## Pipeline
see also: https://www.figma.com/board/wUx9ldLa81defymkUaPdaW/Datamodel-Classifier?node-id=0-1&p=f&t=8c5PUPNB9SZnLb5y-0

everything is based on an existing datamodel
keywords, description: give context of the datamodel
0. start classifier and llm
1. load datamodel
2. map resources
3. map properties( this could mean: it might be that properties cannot be mapped, because the obvious mapping of the property is not possible, because it is not compatible with the already decided mapping of the resource)

 
### In detail
    
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
