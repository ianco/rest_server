extern crate hyper;
extern crate reqwest;

use std::str;
use std::collections::HashMap;
use hyper::header::{Headers, Authorization};


fn main() {
    println!("Hello, world!");

    let client = reqwest::Client::new();

    println!("Try to do an un-authenticated GET list of items");
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

    println!("Try to POST a JSON body un-authenticated");
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

    println!("Try to POST a JSON body using basic auth");
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

    println!("Try to POST a JSON body using a DRF token");
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "888");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut headers4 = Headers::new();
    headers4.set(Authorization("Token 71bee00fa76f08e5f17ceed783a9addd2619bc21".to_owned()));
    let mut res4 = client.post("http://localhost:8000/items/")
        .headers(headers4)
        .json(&map)
        .send()
        .unwrap();
    if res4.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res4.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
    } else if res4.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res4.status());
    }

    println!("Try to POST a JSON body using a JWT token");
}
