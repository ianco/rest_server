extern crate reqwest;

fn main() {
    println!("Hello, world!");

    reqwest::Client::new()
        .get("http://localhost:8000")
        .send()
        .unwrap();

}
