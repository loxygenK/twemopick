use clap::{Arg, ArgMatches, crate_name, crate_authors, crate_version, crate_description, app_from_crate};

pub fn get_command_args<'a>() -> ArgMatches<'a> {
    return app_from_crate!()
        .arg(
            Arg::with_name("emoji")
                .help("Emoji to convert into Twemoji.")
                .required(true)
                .takes_value(true)
        )
        .get_matches()
}
