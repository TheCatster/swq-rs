use clipboard_ext::{prelude::*, x11_fork::ClipboardContext};
use json::{self, parse};
use rand::{seq::SliceRandom, thread_rng};
use ureq::{self, get};

pub fn get_gif(keywords: String) {
    let result = request_gif(&keywords);
    if result.is_err() {
        panic!("Failed getting GIF.");
    }

    let gif = json_to_gif(result.unwrap());
    if gif.is_err() {
        panic!("Failed getting GIF.");
    }
    let gif = gif.unwrap();

    println!("{}", gif);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(gif.to_owned()).unwrap();

    println!("GIF has been added to your clipboard!");
}

fn request_gif(keywords: &String) -> Result<String, ureq::Error> {
    let url: String = format!(
        "https://api.tenor.com/v1/search?key={api_key}&q={quote}&contentfilter=high&limit=30",
        api_key = "",
        quote = format!("star wars {}", keywords)
    );
    let body = get(&url).call()?.into_string()?;
    Ok(body)
}

fn json_to_gif(json: String) -> Result<String, ()> {
    let parsed = parse(&json).unwrap();
    if parsed["results"].members().collect::<Vec<_>>().len() == 0 {
        println!("GIF could not be found with those keywords. Try again!");
        return Err(());
    }
    let random_gif = parsed["results"]
        .members()
        .map(|x| -> String { format!("{}\n", x["url"]) })
        .collect::<Vec<_>>()
        .choose(&mut thread_rng())
        .unwrap()
        .to_string();

    Ok(random_gif)
}
