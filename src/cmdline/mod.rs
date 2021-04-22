use clap::{App, Arg, ArgMatches};

pub fn get_command_args<'a>() -> ArgMatches<'a> {
    return App::new("TwEmoPick")
        .version("1.0.0")
        .author("Flisan <me@loxygen.dev")
        .about("Pick twemoji pictures from Unicode emoji.")
        .arg(
            Arg::with_name("emoji")
                .help("Emoji to convert into Twemoji.")
                .required(true)
                .takes_value(true)
        )
        .get_matches()
}
