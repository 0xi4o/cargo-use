mod args;
mod run;

use args::parse_args;
use run::execute;

fn main() {
    let args = parse_args();

    execute(args);
}
