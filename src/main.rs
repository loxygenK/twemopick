mod cmdline;
mod emoji;

fn main() {

    let args = cmdline::get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    let codepoint = emoji::codepoint::get_emoji_codepoint(emoji);
    let url = emoji::url::get_emoji_dataurl(&codepoint, &emoji::url::ImageType::Raster);

    println!("Selected: {}", &emoji);
    println!("Code: {}", &codepoint);
    println!("Url : {}", &url);


}
