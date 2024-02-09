# Proton Rust Client

Exploring a proof of concept Rust client for [Proton / TimePlus]([url](https://www.timeplus.com/)https://www.timeplus.com/). 

## Install ch2rs

```
cargo install ch2rs
```

## Install proton

```
brew install proton
```

1. Start the Proton server:
   $ proton server start

2. In a separate terminal, connect to the server:
   $ proton client
   (Note: If you encounter a 'connection refused' error, use: proton client --host 127.0.0.1)

3. To terminate the server, press ctrl+c in the server terminal.

   For detailed usage and more information, check out the Timeplus documentation:
   https://docs.timeplus.com/

## Run the client

```
make run
```

## Cargo & Make

Make sure all cargo tools are installed. To do so, just run ```make install```
After all tools have been installed, the following commands are ready to use.
```
make
    make build   	Builds the code base incrementally (fast) for dev.
    make check   	Checks the code base for security vulnerabilities.
    make clean   	Cleans generated files and folders.
    make fix   		Fixes linting issues as reported by clippy.
    make format   	Formats call code according to cargo fmt style.
    make install   	Tests and installs all make script dependencies.
    make release   	Builds the code base for release.
    make test   	Tests across all crates.
    make run   		Runs the default binary.
    make update   	Update rust, pull git, and build the project.
```