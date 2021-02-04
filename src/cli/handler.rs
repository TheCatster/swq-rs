use crate::requests::{gif::get_gif, quote::get_quote};
use clap::ArgMatches;

pub fn handle_commands(matches: ArgMatches) {
    let keywords = match matches.subcommand() {
        ("quote", Some(sub_m)) => String::from(sub_m.value_of("KEYWORDS").unwrap()),
        ("gif", Some(sub_m)) => String::from(sub_m.value_of("KEYWORDS").unwrap()),
        ("add_quote", Some(sub_m)) => String::from(sub_m.value_of("KEYWORDS").unwrap()),
        ("remove_quote", Some(sub_m)) => String::from(sub_m.value_of("KEYWORDS").unwrap()),
        _ => String::from(""),
    };

    let verbose = if matches.is_present("verbose") {
        true
    } else {
        false
    };

    match matches.subcommand() {
        ("quote", Some(_sub_m)) => get_quote(keywords, verbose),
        ("gif", Some(_sub_m)) => get_gif(keywords, verbose),
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
