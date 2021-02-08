use anyhow::Result;

use cli::run;

mod cli;
mod gif;
mod quote;

fn main() -> Result<()> {
    run()
}
