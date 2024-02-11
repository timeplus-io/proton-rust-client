[//]: # (---)
[//]: # (SPDX-License-Identifier: Apache-2.0)
[//]: # (---)

#  Rust Client for Proton SQL Streaming Engine
 
Rust client for [Proton / TimePlus]([url](https://www.timeplus.com/)https://www.timeplus.com/). 

Proton is a streaming SQL engine, a fast and lightweight alternative to Apache Flink, 🚀 powered by ClickHouse. It enables developers to solve streaming data processing, routing and analytics challenges from Apache Kafka, Redpanda and more sources, and send aggregated data to the downstream systems. Proton is the core engine of [Timeplus](https://timeplus.com), which is a cloud native streaming analytics platform.


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


## Install ProtonClient

Add the following to your Cargo.toml:

```
[dependencies]
proton_client = { git = "https://github.com/marvin-hansen/proton-rust-client.git" }
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


## What's next?

To see more examples of using Proton, check out the [examples](examples) folder.

Check out the proton [documentation on the SQL Query syntax](https://docs.timeplus.com/query-syntax). 


## Documentation

You find the full documentation for Proton at [docs.timeplus.com](https://docs.timeplus.com/proton) alongside documentation for the Timeplus (Cloud and BYOC) platform.

We also have a [FAQ](https://docs.timeplus.com/proton-faq/) for detailing how we chose Apache License 2.0, how Proton is related to ClickHouse, what features are available in Proton versus Timeplus, and more.


## Contributing

We welcome your contributions! If you are looking for issues to work on, try looking at [the issue list](FIXLINK).

## Need help?

Join our [Timeplus Community Slack](https://timeplus.com/slack) to connect with Timeplus engineers and other Proton users.

For filing bugs, suggesting improvements, or requesting new features, see the [open issues](FIXLINK) here on GitHub.

## Licensing

Proton Rust Client uses Apache License 2.0. See details in the [LICENSE](LICENSE).

