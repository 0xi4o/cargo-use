
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
