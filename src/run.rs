use crate::args::Arguments;

pub fn execute(args: Arguments) {
    let github_link = format!("https://github.com/{}", args.repo);
    let clone_cmd = match args.name {
        Some(name) => format!("git clone {} {}", github_link, name),
        None => format!("git clone {}", github_link),
    };

    let git_init_cmd = match args.gitinit {
        Some(_) => format!("git init"),
        None => String::from(""),
    };

    let additional_deps_cmd = match args.with {
        Some(with) => format!("cargo add {}", with),
        None => String::from(""),
    };

    let mut final_cmd = clone_cmd.clone();

    if git_init_cmd != "" && additional_deps_cmd != "" {
        final_cmd = format!(
            "{} && {} && {}",
            clone_cmd, git_init_cmd, additional_deps_cmd
        );
    } else if git_init_cmd != "" {
        final_cmd = format!("{} && {}", clone_cmd, git_init_cmd);
    } else if additional_deps_cmd != "" {
        final_cmd = format!("{} && {}", clone_cmd, additional_deps_cmd);
    }

    println!("{final_cmd}");
}
