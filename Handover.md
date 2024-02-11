# Handover 


Suggested steps to do after the repo transfer:

## Update Readme:
- Updated Readme with logo etc.
- Update missing links in Readme. These are links to issue tracker and wiki. Run ctr-F and search for FIXLINK to find them all


## Verify CI / Github actions:

Currently, all GH actions trigger on pull request assuming branch protection is in place (as it should). 

THe following GH actions are pre-configured:

- audit - security audit against a DB of all known vulnerabilities. Runs on every PR. Active by default.
- linter / clippy - Checks against a massive database of known Rust lints. Runs on every PR. Active by default.
- test - Runs all uni tests and all doc tests unless marked as no_run. Runs on every PR. Active by default.
- release - auto release, disabled. See below.

For a reference of Rust CI configs, please consult the documentation:

https://doc.rust-lang.org/cargo/guide/continuous-integration.html


## Enable auto release 

Rust packages are published to the crates.io registry. To do so, the following steps are necessary:

1) Verify the pre-defined meta data in Cargo.toml. Repo & homepage is already set to https://github.com/timeplus-io/proton-rust-client
2) Create a free account on https://crates.io/
3) Create an API token in the dashboard
4) login locally, from a terminal: ```cargo login``` Then at the prompt put in the token specified.
5) Conduct a dry run to see if everything is correct: ```cargo publish --dry-run```


For a reference how to publish crates, please read the documentation:

https://doc.rust-lang.org/cargo/reference/publishing.html