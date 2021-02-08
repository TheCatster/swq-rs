use {
    anyhow::{anyhow, Result},
    clap::{clap_app, crate_authors, crate_description, crate_name, crate_version, ArgMatches},
    clipboard_ext::{prelude::*, x11_fork::ClipboardContext},
};

use crate::{gif::get_gif, quote::get_quote};

pub fn run() -> Result<()> {
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

    handle_commands(matches)
}

fn handle_commands(matches: ArgMatches) -> Result<()> {
    let result = match matches.subcommand() {
        ("quote", Some(sub_m)) => get_quote(
            sub_m
                .value_of("KEYWORDS")
                .expect("KEYWORDS will never be None, as it is required by clap."),
        ),
        ("gif", Some(sub_m)) => get_gif(
            sub_m
                .value_of("KEYWORDS")
                .expect("KEYWORDS will never be None, as it is required by clap."),
        ),
        _ => unreachable!(),
    }?;

    println!("{}\n", result);

    let mut ctx: ClipboardContext =
        ClipboardProvider::new().map_err(|_| anyhow!("Failed to open clipboard."))?;
    ctx.set_contents(result)
        .map_err(|_| anyhow!("Failed to set the clipboard contents."))?;

    println!("Added to your clipboard!");

    Ok(())
}
