use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_native_tls::{TlsAcceptor, native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor}};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Read the DER certificate
    // let cert_der = fs::read("src/test_cert.der")?;
    // // Read the PEM key
    // let key_der = fs::read("src/test_key_pkcs8.der")?;
    // // Create an Identity from the key (DER) and certificate (DER)
    // let identity = Identity::from_pkcs8(&key_der, &cert_der)?;

    let pfx = fs::read("src/identity.pfx")?;
    let identity = Identity::from_pkcs12(&pfx, "")?;   // empty password

    // Build a native_tls::TlsAcceptor from the identity
    let native_acceptor = NativeTlsAcceptor::new(identity)?;

    // Wrap it in tokio_native_tls::TlsAcceptor
    let acceptor = TlsAcceptor::from(native_acceptor);

    let listener = TcpListener::bind("127.0.0.1:8443").await?;
    println!("Async TLS server listening on 127.0.0.1:8443");

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(acceptor, stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(acceptor: TlsAcceptor, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut tls_stream = acceptor.accept(stream).await?;
    let mut buf = vec![0; 1024];
    let n = tls_stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello world!";
    tls_stream.write_all(response.as_bytes()).await?;
    tls_stream.flush().await?;
    Ok(())
}
