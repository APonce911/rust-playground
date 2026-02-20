// https://github.com/rust-native-tls/rust-native-tls
use native_tls::{TlsConnector, Certificate};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs;

fn main() {
    one_way_tls()
}

fn one_way_tls() {
    let cert_der = fs::read("ca_cert.der").unwrap().to_vec();
    let cert = Certificate::from_der(&cert_der).unwrap();
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)           // to trust the server
        .build()
        .unwrap();

    let url = "127.0.0.1:8443";
    let stream = TcpStream::connect(url).unwrap();
    let mut stream = connector.connect("127.0.0.1", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

fn mutual_tls() {
    //TODO - generate client.pfx file

    let client_pfx_bytes = fs::read("client.pfx").unwrap().to_vec();
    let client_identity = Identity::from_pkcs12(client_pfx_bytes, "client_pfx_bytes")?;

    let cert_der = fs::read("ca_cert.der").unwrap().to_vec();
    let cert = Certificate::from_der(&cert_der).unwrap();
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)           // to trust the server
        .identity(client_identity)         // to prove who we are
        .build()
        .unwrap();

    let url = "127.0.0.1:8443";
    let stream = TcpStream::connect(url).unwrap();
    let mut stream = connector.connect("127.0.0.1", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}


