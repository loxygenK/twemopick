mod cmdline;
mod emoji;

fn main() {

    let args = cmdline::get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    let codepoint = emoji::codepoint::get_emoji_codepoint(emoji);
    println!("Selected: {}", &emoji);
    println!("Code: {}", &codepoint);
    println!("Url : {}", emoji::url::get_image_url(&codepoint, emoji::url::ImageType::Svg));

}
