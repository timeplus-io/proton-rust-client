

#  Examples for Rust Client for Proton


## Install Proton

Please install Proton as a standalone server or via Docker. Make sure either port 8123 or 3218 is exposed for `pront-rust-client` to connect and run SQL.
* 8123 is the port to run DDL and queries in historical mode
* 3218 is the port to run DDL and quries in streaming mode by default. You can still wrap the stream with `table(..)` function to query them in historical mode.

Learn more
* [SQL syntax](https://docs.timeplus.com/query-syntax)
* [Proton ports](https://docs.timeplus.com/proton-ports)

### As a single binary

On Linux or Mac, you can install it via `curl https://install.timeplus.com | sh`

On Mac, you can also install it via `brew install proton`

After you get the `proton` binary, you can start the Proton server via `proton server start`

In a separate terminal, connect to the server via `proton client` (Note: If you encounter a 'connection refused' error, use: proton client --host 127.0.0.1)

### As a Docker container

```bash
docker run -d --pull always --name proton -p 8123:8123 -p 8463:8463 ghcr.io/timeplus-io/proton:latest
```

Proton is automatically started with port 8123 and 8463 exposed. Open the terminal of the container, and run `proton client`


For detailed usage and more information, check out the documentation: https://docs.timeplus.com/proton


## Install ProtonClient

Add the following to your Cargo.toml:

```
[dependencies]
proton_client = { git = "https://github.com/marvin-hansen/proton-rust-client.git" }
```

[//]: # ( After Crate release on crates.io)

[//]: # (```)

[//]: # ([dependencies])

[//]: # (proton_client =  { version = "0.1.0"})

[//]: # (```)


## Use ProtonClient

```Rust
use proton_client::prelude::{ProtonClient, Result};

const FN_NAME: &str = "[prepare]:";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{} Start", FN_NAME);

    println!("{} Build client", FN_NAME);
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

In the root folder of `protno-rust-client`

1) Create a stream and insert some data

```
cargo run --example prepare
```

Expected output

```
[prepare]:Start
[prepare]:Build client
[prepare]:Create stream if not exists
[prepare]:Insert data
[prepare]:Count inserted data
[prepare]:Inserted data: 1000
[prepare]:Stop
```

2) Stream some data (fetch) and load all data at once (fetch_all)

```
cargo run --example query
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
[main]:Stop
```

3) Cleanup and delete stream


```
cargo run --example remove
```

Expected output

```
[prepare]:Start
[prepare]:Build client
[prepare]:Delete Stream
[prepare]:Stop
```
