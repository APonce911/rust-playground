fn main() {
    let url = "https://airtonponce.com";
    let response = bitreq::get(url).with_timeout(10).send().unwrap();

    assert!(response.as_str().unwrap().contains("</html>"));
    assert_eq!(200, response.status_code);
    assert_eq!("OK", response.reason_phrase);

    // println!("{}", response.as_str().unwrap())
}
