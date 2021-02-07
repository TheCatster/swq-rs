use anyhow::{Context, Result};
use rand::seq::IteratorRandom;

pub(crate) const QUOTES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

pub fn get_quote(keywords: &str) -> Result<String> {
    let keywords = keywords.to_lowercase();
    let mut rng = rand::thread_rng();

    QUOTES
        .iter()
        .filter(|q| q.to_lowercase().contains(keywords.as_str()))
        .choose(&mut rng)
        .map(|q| q.to_string())
        .context("No quote found for those keywords.")
}
