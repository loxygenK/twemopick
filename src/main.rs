mod cmdline;
mod twemoji;
mod clipboard;

use std::{fs::File, env};

use cmdline::get_command_args;
use twemoji::{get_twemoji_codepoint, generate_remote_url, fetch_emoji, EmojiOutputType};

#[tokio::main]
async fn main() -> Result<(), ()> {

    let args = get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    let output_dir = env::current_dir()
        .map_err(|e| eprintln!("{}", e))?
        .to_str()
        .ok_or_else(|| eprintln!("[!] Couldn't get current directory."))?;

    let data = fetch_emoji(emoji, EmojiOutputType::SVG)
        .await 
        .map_err(|x| eprintln!("{}", x))?;

    println!("Fetched {} bytes.", data.len());
    
    File::create(output_dir);

    Ok(())
}
