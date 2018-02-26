extern crate reqwest;

use std::str;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    let client = reqwest::Client::new();

    let mut res1 = client.get("http://localhost:8000/items/")
        .send()
        .unwrap();
    if res1.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res1.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
    } else if res1.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res1.status());
    }

    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "666");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut res2 = client.post("http://localhost:8000/items/")
        .json(&map)
        .send()
        .unwrap();
    if res2.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res2.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
    } else if res2.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res2.status());
    }

    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "777");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut res3 = client.post("http://localhost:8000/items/")
        .basic_auth("ian", Some("pass1234"))
        .json(&map)
        .send()
        .unwrap();
    if res3.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res3.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
    } else if res3.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res3.status());
    }
}
