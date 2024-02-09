# Proton Rust Client

Exploring aa Rust client for [Proton / TimePlus]([url](https://www.timeplus.com/)https://www.timeplus.com/). 

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