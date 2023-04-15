use std::{env, path::Path, process::Command};

use crate::args::Arguments;

pub fn execute_all(args: Arguments) {
    let repo = args.repo.clone();
    let name = args.name.clone();
    let with = args.with.clone();
    let clone_success = execute_clone(&repo, &name);

    let repo_name = String::from(repo.split('/').next().unwrap());
    if clone_success {
        let cd_success = match &name {
            Some(name) => execute_cd(&name),
            None => execute_cd(&repo_name),
        };

        if cd_success {
            let ca_success = execute_cargo_add(&with);
            println!("{ca_success}");
        }
    }
}

fn execute_clone(repo: &String, name: &Option<String>) -> bool {
    let github_link = format!("https://github.com/{}", repo);

    // TODO: Check if this repo exists first before attempting to clone. If it doesn't exist, throw an error.
    let mut command = Command::new("git");
    match name {
        Some(name) => {
            command.args(["clone", &github_link, name]);
        }
        None => {
            command.args(["clone", &github_link]);
        }
    };

    let status = command.status();
    match status {
        Ok(status) => {
            status.success()
        },
        Err(e) => panic!(
            "Failed to clone repo. Check if the repo exists, there are no typos, and that the repo is public. The process returned this error: {}", e
        )
    }
}

fn execute_cd(dir: &String) -> bool {
    // TODO: The current working directory actually changes but it doesn't reflect in the shell. Fix this.
    if Path::new(dir).exists() {
        env::set_current_dir(dir).is_ok()
    } else {
        false
    }
}

fn execute_cargo_add(deps: &Option<String>) -> bool {
    match deps {
        Some(deps) => {
            let mut mods = deps.split(" ").collect::<Vec<&str>>();
            let mut args = vec!["add"];
            args.append(&mut mods);
            let mut command = Command::new("cargo");
            command.args(args);

            println!("Executing: {:?}", command);
            let status = command.status();
            match status {
                Ok(status) => status.success(),
                Err(e) => {
                    panic!("Failed to install additional dependencies. Process returned: {e}")
                }
            }
        }
        None => false,
    }
}
