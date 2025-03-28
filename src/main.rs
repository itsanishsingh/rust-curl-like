use clap::Parser;
use reqwest::{Client, Error, Response};
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
    client: Client,
}

impl HTTPClient {
    fn new(url: String, method: RESTMethods) -> Self {
        Self {
            url,
            method,
            client: reqwest::Client::new(),
        }
    }

    async fn get(&self) -> Result<Response, Error> {
        let res = self.client.get(self.url.to_owned()).send().await?;
        Ok(res)
    }

    async fn post(&self) -> Result<Response, Error> {
        let res = self.client.post(self.url.clone()).body("").send().await?;
        Ok(res)
    }

    async fn put(&self) -> Result<Response, Error> {
        let res = self.client.put(self.url.clone()).body("").send().await?;
        Ok(res)
    }

    async fn delete(&self) -> Result<Response, Error> {
        let res = self.client.delete(self.url.clone()).body("").send().await?;
        Ok(res)
    }

    async fn process(&self) -> Result<Response, Error> {
        match self.method {
            RESTMethods::GET => Ok(self.get().await?),
            RESTMethods::POST => Ok(self.post().await?),
            RESTMethods::PUT => Ok(self.put().await?),
            RESTMethods::DELETE => Ok(self.delete().await?),
        }
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

    let result = client.process().await;

    match result {
        Ok(body) => println!("{:?}", body),
        Err(err) => println!("{}", err),
    }
}
