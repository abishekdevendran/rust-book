use anyhow::Error;
use dotenvy::dotenv;
use redis::{Commands, Connection};
use select::{document::Document, predicate::Name};
use std::{env, process::exit};

const BASE_URL: &str = "BASE_URL";
const SERVICE_URL_ENV_KEY: &str = "VALKEY_URL";
const PING_KEY: &str = "ping";

async fn parse_page(base_url: &str) -> Result<Vec<Box<str>>, Error> {
    let body = reqwest::get(base_url)
        .await?
        .text()
        .await?;

    let links = Document::from(body.as_str())
        .find(Name("a"))
        .filter_map(|node| node.attr("href"))
        .map(|link| Box::<str>::from(link.to_string()))
        .collect();

    Ok(links)
}

fn init_connection(service_url: &str) -> Connection {
    redis::Client::open(service_url)
        .unwrap_or_else(|err| {
            eprintln!(
                "Error initializing client with service url {}: {}",
                service_url, err
            );
            exit(1);
        })
        .get_connection()
        .unwrap_or_else(|err| {
            eprintln!("Error initializing connection: {}", err);
            exit(1);
        })
}

fn increment_key(mut conn: Connection) -> i32 {
    let key = PING_KEY;
    let temp = conn.get(key).unwrap_or(0);
    let _: () = conn.set(key, temp + 1).unwrap_or_else(|err| {
        eprintln!("Error incrementing key: {}", err);
    });
    temp
}

#[tokio::main]
async fn main() {
    if let Err(e) = dotenv() {
        eprintln!(
            "Warning: Could not load .env file: {}. Relying on system environment variables.",
            e
        );
        exit(1)
    }
    let service_url = env::var(SERVICE_URL_ENV_KEY).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    let base_url = env::var(BASE_URL).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    let conn = init_connection(&service_url);
    println!("{}: {}", PING_KEY, increment_key(conn));
    println!("Hello, world!");

    let links = parse_page(&base_url).await.unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
    for link in links {
        println!("{}", link);
    }
}
