use clap::{arg, value_parser, Arg, ArgAction, Command};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Arguments {
    pub repo: String,
    pub name: Option<String>,
    pub with: Option<String>,
}

pub fn parse_args() -> Arguments {
    let sub_cmd = Command::new("use")
        .about("Cargo subcommand to start a new Rust project from a boilerplate/template repository.")
        .author("Ilango Rajagopal")
        .args([
            Arg::new("repo")
                .help("Github repository to use as a template for the new project. You can either use the full link or <user_or_org_slug>/<repo-name>.")
                .required(true),
            arg!(-n --name <NAME> "Name of the project to create from the given repo").value_parser(value_parser!(String)),
            Arg::new("with")
                .help("List of additional dependencies to add the project. Space-separated, surround by quotes.")
                .short('w')
                .long("with")
                .value_name("WITH")
                .value_delimiter(' ')
                .value_delimiter(',')
                .action(ArgAction::Append)
        ]);
    let cmd = Command::new("cargo")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(sub_cmd);

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("use", matches)) => matches,
        _ => unreachable!("shouldn't get here"),
    };

    Arguments {
        repo: String::from(matches.get_one::<String>("repo").unwrap()),
        name: matches.get_one::<String>("name").cloned(),
        with: matches.get_one::<String>("with").cloned(),
    }
}
