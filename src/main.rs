mod cmdline;
mod twemoji;

use cmdline::get_command_args;
use twemoji::{get_twemoji_codepoint, generate_remote_url, fetch_emoji, EmojiOutputType};

#[tokio::main]
async fn main() -> Result<(), ()> {

    let args = get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    println!("Selected: {}", &emoji);
    println!("Code: {}", get_twemoji_codepoint(emoji));
    println!("Remote: {}", generate_remote_url(emoji, EmojiOutputType::SVG));
    println!("--> Trying to fetch...");

    let data = fetch_emoji(emoji, EmojiOutputType::SVG).await;
    if let Err(e) = data {
        println!("Error occurred when trying to fetch emoji:");
        println!("{:?}", e);
        return Err(());
    }
    let data = data.unwrap();

    println!("Fetched {} bytes.", data.len());

    Ok(())
}
