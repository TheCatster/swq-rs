use super::commands::Command;

pub fn handle_command(cmd: &Command) {
    match cmd {
        Command::Quote { keywords, verbose } => {
            if *verbose {
                println!("Verbose Output!");
            }
            println!("Your quote is {}.", keywords);
        }
        Command::Gif { keywords, verbose } => {
            if *verbose {
                println!("Verbose Output!");
            }
            println!("Your GIF is {}.", keywords);
        }
    }
}
