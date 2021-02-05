use super::handler::handle_commands;
use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version, ArgMatches};

pub fn run() {
    let matches: ArgMatches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@subcommand quote =>
                (about: "Retrieves Star Wars quotes")
                (@arg KEYWORDS: +required "Sets the keywords to search quotes for")
        )
        (@subcommand gif =>
                (about: "Retrieves Star Wars GIFs")
                (@arg KEYWORDS: +required "Sets the keywords to search GIFs for")
        )
    )
    .get_matches();

    handle_commands(matches);
}
