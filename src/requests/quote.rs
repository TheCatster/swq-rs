use clipboard_ext::{prelude::*, x11_fork::ClipboardContext};
use rand::{seq::SliceRandom, thread_rng};
pub(crate) const QUOTES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

pub fn get_quote(keywords: String) {
    let random_quote: &str = QUOTES
        .iter()
        .filter(|x| x.to_lowercase().contains(keywords.to_lowercase().as_str()))
        .collect::<Vec<_>>()
        .choose(&mut thread_rng())
        .map_or("No quote found for those keywords.", |s| **s);

    println!("{}\n", random_quote);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(random_quote.to_owned()).unwrap();

    println!("Quote has been added to you clipboard!");
}
