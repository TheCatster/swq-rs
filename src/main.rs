use cli::{commands::Command, handler::handle_command};
use structopt::StructOpt;

mod cli;

fn main() {
    let opt = Command::from_args();
    handle_command(&opt);
}
