use crate::requests::{gif::get_gif, quote::get_quote};
use clap::ArgMatches;

pub fn handle_commands(matches: ArgMatches) {
    let keywords: &str = match matches.subcommand() {
        ("quote", Some(sub_m)) => sub_m.value_of("KEYWORDS").unwrap(),
        ("gif", Some(sub_m)) => sub_m.value_of("KEYWORDS").unwrap(),
        ("add_quote", Some(sub_m)) => sub_m.value_of("KEYWORDS").unwrap(),
        ("remove_quote", Some(sub_m)) => sub_m.value_of("KEYWORDS").unwrap(),
        _ => "",
    };

    match matches.subcommand() {
        ("quote", Some(_sub_m)) => get_quote(keywords),
        ("gif", Some(_sub_m)) => get_gif(keywords),
        ("add_quote", Some(_sub_m)) => {
            println!("This isn't implemented yet... how did you get here?")
        }
        ("remove_quote", Some(_sub_m)) => {
            println!("This isn't implemented yet... how did you get here?")
        }
        _ => println!(
            "This is not a valid command. Please run the help command to see possible options."
        ),
    }
}
