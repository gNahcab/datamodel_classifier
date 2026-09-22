use serde_json::json;

fn example() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let body = json!({
        "messages": [
            {
                "role": "system",
                "content": "You are a CIDOC CRM classifier."
            },
            {
                "role": "user",
                "content": r#"
Which CIDOC CRM class best describes the expected value of this property?

Property: hasBattle
Description: A battle in which a person participated.

Candidates:

E5 Event:
A distinct, delimited and coherent process.
Examples include battles, conferences, births and deaths.

E21 Person:
A real human individual.

E53 Place:
An identifiable extent in space.

Return only the class identifier.
"#
            }
        ],
        "temperature": 0,
        "max_tokens": 20
    });

    let response: serde_json::Value = client
        .post("http://127.0.0.1:8081/v1/chat/completions")
        .json(&body)
        .send()?
        .error_for_status()?
        .json()?;

    println!(
        "{}",
        response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No answer")
    );

    Ok(())
}
