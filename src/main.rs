mod args;

use args::parse_args;

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
