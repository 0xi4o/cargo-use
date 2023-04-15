mod args;
mod run;

use args::parse_args;
use run::execute_all;

fn main() {
    let args = parse_args();

    execute_all(args);
}
