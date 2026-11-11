use serde::Deserialize;

use crate::utils::Error;

fn uri(part: &str) -> String {
    format!("https://safebooru.org/{}", part) // this is lowk kind of useless
}

#[derive(Deserialize)]
pub struct Listing {
    pub file_url: String,
    pub source: String,
    pub id: u32,
    pub tags: String,
}

pub async fn get_random_images(tags: &Vec<String>, limit: &i32) -> Result<Vec<Listing>, Error> {
    let url = uri("index.php?page=dapi&s=post&q=index&limit=1&json=1");
    let client = reqwest::Client::new();
    let q = &[
        ("tags", tags.join(" ")),
        ("limit", limit.to_string()),
        ("json", "1".into()),
    ];

    let results: Vec<Listing> = serde_json::from_str(
        &client
            .get(url)
            .query(q)
            .send()
            .await?
            .text()
            .await
            .unwrap_or("[]".into()),
    )?;

    Ok(results)
}
