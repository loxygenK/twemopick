mod cmdline;
mod emoji;

use cmdline::get_command_args;
use emoji::get_emoji_codepoint;

fn main() {

    let args = get_command_args();
    let emoji = args.value_of("emoji").expect("wtf - required didn't work");
    println!("Selected: {}", &emoji);
    println!("Code: {}", get_emoji_codepoint(emoji));

}
