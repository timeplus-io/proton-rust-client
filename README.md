[//]: # (---)
[//]: # (SPDX-License-Identifier: Apache-2.0)
[//]: # (---)

#  Rust Client for Proton

Rust client for [Timeplus Proton](https://github.com/timeplus-io/proton).

Proton is a streaming SQL engine, a fast and lightweight alternative to Apache Flink, 🚀 powered by ClickHouse. It enables developers to solve streaming data processing, routing and analytics challenges from Apache Kafka, Redpanda and more sources, and send aggregated data to the downstream systems. Proton is the core engine of [Timeplus](https://timeplus.com), which is a cloud native streaming analytics platform.

The initial version(0.1.0) of the client was written by [Marvin Hansen](https://github.com/marvin-hansen). Thanks for the contribution.

This client uses https://crates.io/crates/clickhouse as a dependency.

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
proton_client = { git = "https://github.com/timeplus-io/proton-rust-client.git" }
```

[//]: # (AFTER the release of the package on crates.io)

[//]: # (Add the proton client to your project by running in a terminal:)

[//]: # ()
[//]: # (```)

[//]: # (cargo add proton_client)

[//]: # (```)

[//]: # ()
[//]: # (of by adding the following to your Cargo.toml:)

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
    println!("{}Start", FN_NAME);

    println!("{}Build client", FN_NAME);
    let client = ProtonClient::new("http://localhost:8123");

    println!("{}Create stream if not exists", FN_NAME);
    create_stream(&client)
        .await
        .expect("[main]: Failed to create Stream");

    println!("{}Stop", FN_NAME);
    Ok(())
}

pub async fn create_stream(client: &ProtonClient) -> Result<()> {
     client
         .execute_query("CREATE STREAM IF NOT EXISTS test_stream(no uint32, name string) ORDER BY no")
         .await
}
```

## Run the examples

```bash
make example
```

Which shows:

```text
-----------------------------------------
Select the number of the example to run:
-----------------------------------------
1) prepare: Prepare Proton for the examples: create table & load data
2) query: Query Proton with sample queries
3) remove: Cleanup Proton and delete streams
4) quit: Exit

-----------------------------------------
Make sure Proton is running
-----------------------------------------
```

Please select the number of the example to run. Also,
first run the prepare example to prepare Proton for the query example.
See the code of all examples [here](examples).

## What's next?

To see more examples of using Proton, check out the [examples](https://github.com/timeplus-io/proton/tree/develop/examples) folder in Proton repo.

Please note, by default the SQL queries are in streaming mode. Learn more about SQL syntax at [Proton Documentation](https://docs.timeplus.com/query-syntax).


## Documentation

You find the full documentation for Proton at [docs.timeplus.com](https://docs.timeplus.com/proton) alongside documentation for the Timeplus (Cloud and BYOC) platform.

## Contributing

We welcome your contributions!

## Need help?

Join our [Timeplus Community Slack](https://timeplus.com/slack) to connect with Timeplus engineers and other Proton users.

For filing bugs, suggesting improvements, or requesting new features, see the [open issues](https://github.com/timeplus-io/proton-rust-client/issues) here on GitHub.

## Licensing

Proton Rust Client uses Apache License 2.0. See details in the [LICENSE](LICENSE).
