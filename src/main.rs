mod cmdline;

use cmdline::get_command_args;

fn main() {
    let args = get_command_args();
    println!("Selected: {}", args.value_of("emoji").unwrap());
}
