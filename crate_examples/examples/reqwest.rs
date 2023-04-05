use reqwest::Client;

pub struct C {
    client: Client,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let res = client.get("https://www.rust-lang.org").send().await;
    println!("{:#?}", res);
}
