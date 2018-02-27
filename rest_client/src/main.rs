extern crate hyper;
extern crate serde_json;
extern crate reqwest;

use std::str;
use std::collections::HashMap;
use hyper::header::{Headers, Authorization, Basic};
use serde_json::{Value, Error};


fn main() {
    println!("Hello, world!");

    let client = reqwest::Client::new();

    println!("Try to do an un-authenticated GET list of items");
    let res1 = client.get("http://localhost:8000/items/")
        .send()
        .unwrap();
    check_result(res1);

    println!("Try to POST a JSON body un-authenticated");
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "666");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let res2 = client.post("http://localhost:8000/items/")
        .json(&map)
        .send()
        .unwrap();
    check_result(res2);

    println!("Try to POST a JSON body using basic auth");
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "777");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut headers3 = Headers::new();
    headers3.set(Authorization(Basic {
           username: "ian".to_owned(),
           password: Some("pass1234".to_owned())
       }));
    let res3 = client.post("http://localhost:8000/items/")
        .headers(headers3)
        .json(&map)
        .send()
        .unwrap();
    check_result(res3);

    println!("Try to POST a JSON body using a DRF token");
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "888");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut headers4 = Headers::new();
    headers4.set(Authorization("Token 71bee00fa76f08e5f17ceed783a9addd2619bc21".to_owned()));
    let res4 = client.post("http://localhost:8000/items/")
        .headers(headers4)
        .json(&map)
        .send()
        .unwrap();
    check_result(res4);

    println!("Try to register a new user using a JWT token");
    let mut map = HashMap::new();
    map.insert("username", "ian66");
    map.insert("password1", "pass1234");
    map.insert("password2", "pass1234");
    let res5 = client.post("http://localhost:8000/rest-auth/registration/")
        .json(&map)
        .send()
        .unwrap();
    check_result(res5);

    println!("Try to login using a JWT token");
    let mut map = HashMap::new();
    map.insert("username", "ian66");
    map.insert("password", "pass1234");
    let res6 = client.post("http://localhost:8000/rest-auth/login/")
        .json(&map)
        .send()
        .unwrap();
    let jwt_token = check_result_jwt(res6);

    println!("Try to POST a JSON body using a JWT token");
    let mut map = HashMap::new();
    map.insert("wallet_name", "Rust_Wallet");
    map.insert("item_type", "rust_claim");
    map.insert("item_id", "999");
    map.insert("item_value", "{\"this\":\"is\", \"a\":\"claim\", \"from\":\"rust\"}");
    let mut headers7 = Headers::new();
    let mut auth_str = "JWT ".to_owned();
    auth_str.push_str(&jwt_token);
    headers7.set(Authorization(auth_str));
    let res7 = client.post("http://localhost:8000/items/")
        .headers(headers7)
        .json(&map)
        .send()
        .unwrap();
    check_result(res7);

    println!("Try to GET a JSON body using a JWT token");
    let mut headers8 = Headers::new();
    let mut auth_str = "JWT ".to_owned();
    auth_str.push_str(&jwt_token);
    headers8.set(Authorization(auth_str));
    let res8 = client.get("http://localhost:8000/items/")
        .headers(headers8)
        .send()
        .unwrap();
    check_result(res8);
}

fn check_result(mut res: reqwest::Response) {
    if res.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }
}

fn check_result_jwt(mut res: reqwest::Response) -> String {
    if res.status().is_success() {
        println!("success!");
        let mut buf: Vec<u8> = vec![];
        let _cres = res.copy_to(&mut buf);
        let s = match str::from_utf8(buf.as_slice()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(s).unwrap();

        // Access parts of the data by indexing with square brackets.
        let ss = v["token"].as_str().unwrap();
        return ss.to_owned()
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }
    "".to_owned()
}
