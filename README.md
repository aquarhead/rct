RabbitMQ Connection Tester, pronounced as "rekt".

- Requires native TLS support (`openssl` etc..)
- Start with `RUST_LOG=rct ./rct`, or in PowerShell: `$ENV:RUST_LOG = "rct"`
  - This only prints logs from `rct` itself, toggle logs from internal crates if necessary following [the `env_logger` documentation](https://docs.rs/env_logger/0.7.1/env_logger/).
