
# cargo-use

Cargo subcommand to start a new Rust project from a boilerplate/template repository.


## Prerequisites

- `git`
## Installation

Install cargo-use with cargo

```bash
cargo install cargo-use
```
    
## Usage

```bash
cargo use <user_or_org_name>/<repo-name>
```
or
```bash
cargo use https://github.com/<user_or_org_name>/<repo-name>
```

Default:
```bash
cargo use i4o-dev/startrs
```

Custom project name:
```bash
cargo use i4o-dev/startrs --name test-project
```

With additional dependencies:
```bash
cargo use i4o-dev/startrs --with tokio axum
```

## TODO

- [ ] Change name in `Cargo.toml` if name is given
- [ ] Repo could include full URL or just `<username>/<repo>` format. Both should work.
- [ ] Unset git remote after the repo is created
