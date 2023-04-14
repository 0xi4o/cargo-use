use execute::Execute;
use std::process::Command;

use crate::args::Arguments;

pub fn execute(args: Arguments) {
    let github_link = format!("https://github.com/{}", &args.repo);

    let mut clone_cmd = Command::new("git");

    match &args.name {
        Some(name) => {
            clone_cmd.arg("clone");
            clone_cmd.arg(github_link);
            clone_cmd.arg(name);
        }
        None => {
            clone_cmd.arg("clone");
            clone_cmd.arg(github_link);
        }
    };

    let clone_output = clone_cmd.execute_output().unwrap();
    // TODO: do something with this output later?
    println!("{:?}", clone_output);

    // TODO: Shit works till this point and stops working from here on. Try chaining all of the commands together.

    // let mut ls_cmd = Command::new("ls");
    // let _ = ls_cmd.output().expect("failed");

    // let name = args.name.unwrap().clone();
    // if name != "" {
    //     let mut cd_cmd = Command::new("cd");
    //     cd_cmd.arg("test");
    //     let cd_output = cd_cmd.execute_output().unwrap();
    //     println!("{:?}", cd_output);
    // } else {
    //     let mut cd_cmd = Command::new("cd");
    //     let repo_name = args.repo.clone();
    //     let mut repo_parts = repo_name.split("/");
    //     let dir_name = repo_parts.next().unwrap();
    //     cd_cmd.arg(dir_name);
    // }

    // if args.gitinit.unwrap() {
    //     let mut git_init_cmd = Command::new("git");
    //     git_init_cmd.arg("init");
    //     let git_init_output = git_init_cmd.execute_output().unwrap();
    //     // TODO: do something with this output later?
    //     println!("{:?}", git_init_output);
    // }

    // let additional_deps = args.with.unwrap().clone();
    // if additional_deps != "" {
    //     let mut cargo_cmd = Command::new("cargo");
    //     cargo_cmd.arg("add");
    //     cargo_cmd.arg(additional_deps);
    //     let cargo_output = cargo_cmd.execute_output().unwrap();
    //     // TODO: do something with this output later?
    //     println!("{:?}", cargo_output);
    // }
}
