use clap::{Arg, ArgMatches, crate_name, crate_authors, crate_version, crate_description, app_from_crate};

pub fn get_command_args<'a>() -> ArgMatches<'a> {
    return app_from_crate!()
        .arg(
            Arg::with_name("emoji")
                .help("Emoji to convert into Twemoji.")
                .required(true)
                .takes_value(true)
                .validator(
                    |x| if x.chars().count() == 1 {
                        Ok(())
                    } else {
                        Err("only one character is allowed".to_string())
                    }
                )
        )
        .arg(
            Arg::with_name("use_svg")
                .help("Get the SVG data URL.")
                .short("s")
                .long("svg")
                .conflicts_with("use_png")
        )
        .arg(
            Arg::with_name("use_png")
                .help("Get the PNG data URL.")
                .short("p")
                .long("png")
        )
        .get_matches()
}
