use anyhow::{Context, Result};
use json::{self, parse};
use rand::seq::IteratorRandom;

const BASE_URL: &str = "https://api.tenor.com/v1/search";

pub fn get_gif(keywords: &str) -> Result<String> {
    let result = request_gif(keywords)?;
    json_to_gif(result)
}

fn request_gif(keywords: &str) -> Result<String> {
    ureq::get(BASE_URL)
        .query("key", "") // TODO Use actual token from user
        .query("q", format!("star wars {}", keywords).as_str())
        .query("contentfilter", "high")
        .query("limit", "30")
        .call()
        .context("Failed to request for gif")?
        .into_string()
        .context("Failed to make string for response")
}

fn json_to_gif(json: String) -> Result<String> {
    let parsed = parse(&json).unwrap();
    let mut rng = rand::thread_rng();

    parsed["results"]
        .members()
        .choose(&mut rng)
        .map(|j| j["url"].to_string())
        .context("GIF could not be found with those keywords. Try again!")
}
