use clap::{Arg, Command};

fn main() {
    let matches = Command::new("gcal")
        .about("Google Calendar - CLI")
        .version("0.0.1")
        .arg(Arg::new("title")
            .help("Sets the event title")
            .required(true))
        .arg(Arg::new("date")
            .help("Sets the event date")
            .required(false)).
        get_matches();

    println!("-title used: {:?}", matches.get_one::<String>("title"));
    println!("-date used: {:?}", matches.get_one::<String>("date"));
}
