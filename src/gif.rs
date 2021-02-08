use anyhow::{anyhow, Context, Result};
use json::{self, parse};
use keyring;
use rand::seq::IteratorRandom;
use std::{env, io::stdin};

const BASE_URL: &str = "https://api.tenor.com/v1/search";

pub fn get_gif(keywords: &str) -> Result<String> {
    let api_key = authenticate_api().expect("Unable to interpret API key.");
    let result = request_gif(keywords, &api_key)?;
    json_to_gif(&result)
}

fn request_gif(keywords: &str, api_key: &str) -> Result<String> {
    ureq::get(BASE_URL)
        .query("key", api_key)
        .query("q", format!("star wars {}", keywords).as_str())
        .query("contentfilter", "high")
        .query("limit", "30")
        .call()
        .context("Failed to request for gif")?
        .into_string()
        .context("Failed to make string for response")
}

fn json_to_gif(json: &str) -> Result<String> {
    let parsed = parse(&json).unwrap();
    let mut rng = rand::thread_rng();

    parsed["results"]
        .members()
        .choose(&mut rng)
        .map(|j| j["url"].to_string())
        .context("GIF could not be found with those keywords. Try again!")
}

fn authenticate_api() -> Result<String> {
    let service = "swq-rs";
    let username = env::var("USER").unwrap_or_else(|_| String::from("swq"));
    let keyring = keyring::Keyring::new(&service, &username);
    let password = keyring.get_password();
    if password.is_err() {
        let mut input = String::new();
        println!("Please enter your Tenor API key: ");
        stdin().read_line(&mut input)?;
        keyring
            .set_password(input.trim())
            .map_err(|_| anyhow!("Cannot set API token."))?;
    }
    keyring
        .get_password()
        .map_err(|_| anyhow!("Cannot get API token."))
}
