mod cmdline;
mod emoji;

fn main() {

    let args = cmdline::get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    let codepoint = emoji::codepoint::get_emoji_codepoint(emoji);
    let url = emoji::url::get_image_url(&codepoint, emoji::url::ImageType::Svg);

    println!("Selected: {}", &emoji);
    println!("Code: {}", &codepoint);
    println!("Url : {}", &url);

    let image_bytes = reqwest::blocking::get(&url)
        .expect("Receiving emoji image data")
        .error_for_status()
        .expect("Checking acceptance of the request")
        .bytes()
        .expect("Converting the response into byte");

    println!("Length: {} bytes", image_bytes.len());
    println!("Encoded:{:?}", image_bytes);

}
