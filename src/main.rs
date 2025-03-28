use clap::Parser;
use reqwest::Error;
use std::process;

// const BASE_URL: &str = "https://dummy.restapiexample.com/api/v1/employees";

#[derive(Debug)]
enum RESTMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

impl RESTMethods {
    fn from_str(method: String) -> Result<Self, String> {
        match method.trim() {
            "GET" => Ok(RESTMethods::GET),
            "POST" => Ok(RESTMethods::POST),
            "PUT" => Ok(RESTMethods::PUT),
            "DELETE" => Ok(RESTMethods::DELETE),
            _ => Err("Method not found".to_owned()),
        }
    }
}

/// Curl like cli app
#[derive(Parser)]
struct Args {
    /// The url
    url: String,
    /// The RESTmethod
    method: String,
}

#[derive(Debug)]
struct HTTPClient {
    url: String,
    method: RESTMethods,
}

impl HTTPClient {
    fn new(url: String, method: RESTMethods) -> Self {
        Self { url, method }
    }

    async fn get(&self) -> Result<String, Error> {
        let body = reqwest::get(self.url.clone()).await?.text().await?;
        Ok(body)
    }

    async fn post(&self) -> Result<String, Error> {
        let body = reqwest::get(self.url.clone()).await?.text().await?;
        Ok(body)
    }

    async fn put(&self) -> Result<String, Error> {
        let body = reqwest::get(self.url.clone()).await?.text().await?;
        Ok(body)
    }

    async fn delete(&self) -> Result<String, Error> {
        let body = reqwest::get(self.url.clone()).await?.text().await?;
        Ok(body)
    }

    async fn process(&self) -> Result<String, Error> {
        match self.method {
            RESTMethods::GET => Ok(self.get().await?),
            RESTMethods::POST => Ok(self.post().await?),
            RESTMethods::PUT => Ok(self.put().await?),
            RESTMethods::DELETE => Ok(self.delete().await?),
        }
    }

    fn print(&self) {
        println!(
            "The url is {} and the method is {:?}.",
            self.url, self.method
        );
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let url = args.url;
    let method = args.method;

    let method = match RESTMethods::from_str(method.to_ascii_uppercase()) {
        Ok(method) => method,
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    };

    let client = HTTPClient::new(url.to_owned(), method);

    client.print();

    let result = client.process().await;

    match result {
        Ok(body) => println!("{}", body),
        Err(err) => println!("{}", err),
    }
}
