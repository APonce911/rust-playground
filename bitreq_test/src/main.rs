#[cfg(any(feature = "rustls", feature = "native-tls"))]
fn main() {
    let url = "https://airtonponce.com";
    
    one_shot_request(&url).expect("failed request");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .expect("failed to build Tokio runtime");

    let _ = runtime.block_on(request_with_client(&url));
}

fn one_shot_request(url: &str) -> Result<(), bitreq::Error>{
    let response = bitreq::get(url).with_timeout(10).send()?;

    assert!(response.as_str().unwrap().contains("</html>"));
    assert_eq!(200, response.status_code);
    assert_eq!("OK", response.reason_phrase);

    println!("{}", response.status_code);

    Ok(())
}

async fn request_with_client(url: &str) -> Result<(), bitreq::Error>{
    let client = bitreq::Client::new(1);
    let response = client.send_async(bitreq::get(url)).await.unwrap();

    assert!(response.as_str().unwrap().contains("</html>"));
    assert_eq!(200, response.status_code);
    assert_eq!("OK", response.reason_phrase);

    println!("{}", response.status_code);
    Ok(())
}
