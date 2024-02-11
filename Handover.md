[//]: # (---)
[//]: # (SPDX-License-Identifier: Apache-2.0)
[//]: # (---)

# Handover 

Suggested steps to do after the repo transfer:

## Add the active maintainer 

Add the new / active maintainer to the following files: MAINTAINERS.md, CODEOWNERS, and CONTRIBUTING.md.

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


## Publish the client to crates.io

Rust packages are published to the crates.io registry. To do so, the following steps are necessary:

1) Ensure all changes are comited and pushed to origin. This is important otherwise publish will abort.
2) Verify the pre-defined meta data in Cargo.toml. Repo & homepage is already set to https://github.com/timeplus-io/proton-rust-client
3) Create a free account on https://crates.io/
4) Create an API token in the dashboard
5) login locally, from a terminal: ```cargo login``` Then at the prompt put in the token specified. 
6) Conduct a dry run to see if everything is correct: ```cargo publish --dry-run```
7) When the dry run completes without error, publish the first version: ```cargo publish```

For a reference how to publish crates, please read the documentation:

https://doc.rust-lang.org/cargo/reference/publishing.html

## Enable auto-release

The conventional Rust release process requires a fair amount of manual steps, such as bumping up version number, 
adding new git tags, generating changelog, publishing to crates.io etc. For full automation, the release-plz system
has already been pre-configured. 

Release-plz automates your relase process:

* CHANGELOG generation (with git-cliff).
* Creation of GitHub/Gitea releases.
* Publishing to a cargo registry (crates.io by default).
* Version bumps in Cargo.toml.

Release-plz updates your packages with a release Pull Request based on:
* Your git history, following Conventional commits.
* API breaking changes (detected by cargo-semver-checks).

To enable the release-plz, edit the rust-release file in .github/workflows, uncomment the release-plz section,
and add your crates.io API token as a Github secret. 

Once the release-plz is enabled, every merged PR is added to an automatically generated release PR. When the release PR
is merged, a new release is fully automatically published to the cargo registry and the documentation is uploaded to rustdocs.org.

For details on how to configure relese-plz, consult the documentation:
https://github.com/MarcoIeni/release-plz

For details of how to add a GH secret, see the official GH documentation:

https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions