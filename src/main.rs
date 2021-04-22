mod cmdline;
mod twemoji;

use cmdline::get_command_args;
use twemoji::{get_twemoji_codepoint, generate_remote_url, EmojiOutputType};

fn main() {

    let args = get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    println!("Selected: {}", &emoji);
    println!("Code: {}", get_twemoji_codepoint(emoji));
    println!("Remote: {}", generate_remote_url(emoji, EmojiOutputType::SVG));

}
