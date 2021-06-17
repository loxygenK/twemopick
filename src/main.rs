mod cmdline;
mod emoji;

fn main() {

    let args = cmdline::parse_argument();
    let url = emoji::url::get_emoji_dataurl(
        &emoji::codepoint::get_emoji_codepoint(&args.character),
        &args.image_type
    );

    print!("{}", &url);
}
