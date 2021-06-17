use clap::{Arg, ArgMatches, crate_name, crate_authors, crate_version, crate_description, app_from_crate};

use crate::emoji::url::ImageType;

pub struct Argument {
    pub image_type: ImageType,
    pub character: String
}

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

pub fn parse_argument() -> Argument {
    let args = get_command_args();
    let character = args.value_of("emoji").expect("wtf - required didn't work").to_string();
    let use_svg = args.is_present("use_svg");
    let use_png = args.is_present("use_png");

    let image_type = match (use_svg, use_png) {
        (true, true) => panic!("wtf - conflict didn't work"),
        (false, true) => ImageType::Raster,
        (true, false) => ImageType::Svg,
        (false, false) => ImageType::Svg
    };

    Argument { character, image_type }
}
