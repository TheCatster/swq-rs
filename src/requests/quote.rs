pub(crate) const QUOTES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/quotes.rs"));

pub fn get_quote(keywords: String, verbose: bool) {
    println! {"Verbose? {}", verbose};
    println!("Getting quote for {}", keywords);
    let possible_quotes: Vec<String> = QUOTES
        .iter()
        .filter(|x| x.to_lowercase().contains(keywords.to_lowercase().as_str()))
        .map(|x| x.to_string())
        .collect();

    println!("{:?}", possible_quotes);
}
