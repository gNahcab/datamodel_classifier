use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbeddingRequest {
    input: Vec<String>,
    model: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();

    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    dot / (norm_a * norm_b)
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let texts = vec![
        "search_query: TextValue, hasBattle".to_string(),

        "search_document: E5 Event. A distinct, delimited and coherent process. \
         Examples include battles, conferences, births, deaths and disasters."
            .to_string(),

        "search_document: E21 Person. A real human individual.".to_string(),

        "search_document: E53 Place. An identifiable extent in space.".to_string(),
    ];

    let request = EmbeddingRequest {
        input: texts,
        model: "nomic-embed-text-v1.5".to_string(),
    };

    let response: EmbeddingResponse = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:8080/v1/embeddings")
        .json(&request)
        .send()?
        .error_for_status()?
        .json()?;

    let query = &response.data[0].embedding;

    let classes = [
        ("E5 Event", &response.data[1].embedding),
        ("E21 Person", &response.data[2].embedding),
        ("E53 Place", &response.data[3].embedding),
    ];

    let mut results: Vec<_> = classes
        .iter()
        .map(|(name, embedding)| {
            let score = cosine_similarity(query, embedding);
            (*name, score)
        })
        .collect();

    results.sort_by(|a, b| b.1.total_cmp(&a.1));

    for (name, score) in &results {
        println!("{name:12} {score:.4}");
    }

    println!();
    println!("Best match: {}", results[0].0);

    Ok(())
}
