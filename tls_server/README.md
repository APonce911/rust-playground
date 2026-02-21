## TLS server

I've used this local server to test the https custom certificate implementation on [bitreq crate](https://github.com/rust-bitcoin/corepc/tree/master/bitreq)

### Server run

with [rustls](https://github.com/rustls/rustls)

    cargo run --features tls-rustls

alternatively, run with [native-tls](https://github.com/rust-native-tls/rust-native-tls)

    cargo run --features tls-native

### Client Run (one-way TLS)

Use tls/ca_cert.der file

Bitreq Example:

    #[tokio::test]
    #[cfg(all(feature = "native-tls", feature = "tokio-native-tls"))]
    async fn test_https_local() {
        let cert_der = include_bytes!("ca_cert.der");
        let client =
            bitreq::Client::builder().with_root_certificate(cert_der.as_slice()).unwrap().build();
        let response = client.send_async(bitreq::get("https://127.0.0.1:8443")).await.unwrap();
        assert_eq!(response.status_code, 200);
    }


### Client Run (mutual TLS)

native-tls Example:

    TODO

### How to generate new Keys / Certificates

Read [here](https://github.com/APonce911/rust-playground/blob/main/tls_server/TLS.md)  
