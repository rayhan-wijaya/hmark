use clap::{command, Arg, ArgMatches};

fn get_matches() -> ArgMatches {
    command!()
        .arg(
            Arg::new("set")
                .short('s')
                .long("set")
                .value_name("key=url")
                .help("Sets a bookmark using a provided key, and url."),
        )
        .get_matches()
}

fn main() {
    todo!();
}
