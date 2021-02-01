use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "star wars quotes -")]
pub enum Command {
    #[structopt(name = "quote")]
    Quote {
        keywords: String,
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(name = "gif")]
    Gif {
        keywords: String,
        #[structopt(short, long)]
        verbose: bool,
    },
}
