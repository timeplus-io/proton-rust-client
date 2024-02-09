# Proton Rust Client

Exploring a proof of concept Rust client for [Proton / TimePlus]([url](https://www.timeplus.com/)https://www.timeplus.com/). 


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


## Build a ProtonClient

```Rust
use proton::prelude::{ProtonClient, Result};

const FN_NAME: &str = "[prepare]:";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{} Start", FN_NAME);

    println!("{}Build client", FN_NAME);
    let client = ProtonClient::new("http://localhost:8123");

    println!("{} Create stream if not exists", FN_NAME);
    create_stream(&client)
        .await
        .expect("[main]: Failed to create Stream");

    println!("{} Stop", FN_NAME);
    Ok(())
 }
```


## Run the client code example

1) Create a stream and insert some data

```
cargo run --bin prepare
```

Expected output

```
[prepare]: Start
[prepare]:Build client
[prepare]: Create stream if not exists
[prepare]:Insert data
[prepare]:Count inserted data
[prepare]:Inserted data: 1000
[prepare]: Stop
```

2) Stream some data (fetch) and load all data at once (fetch_all)

```
cargo run --bin main
```

Expected output

```
[main]:Build client
[main]:Fetch data
MyRow { no: 500, name: "foo" }
MyRow { no: 501, name: "foo" }
MyRow { no: 502, name: "foo" }
MyRow { no: 503, name: "foo" }
MyRow { no: 504, name: "foo" }
[main]:Fetch all data
[MyRowOwned { no: 500, name: "foo" }, MyRowOwned { no: 501, name: "foo" }, MyRowOwned { no: 502, name: "foo" }, MyRowOwned { no: 503, name: "foo" }, MyRowOwned { no: 504, name: "foo" }]
[main]: Stop
```

3) Cleanup and delete stream


```
cargo run --bin cleanup
```

Expected output

```
[prepare]: Start
[prepare]:Build client
[prepare]: Delete Stream
[prepare]: Stop
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