# IDE setup
We are currently using unstable rustfmt features in our rustfmt.toml, which isn't supported through rustfmt stable natively, as rustfmt v1 stable only supports unstable options via the command line. As such we are currently using a small wrapper around rustfmt, called `rustfmt-unstable`, which essentially passes the arguments in the config file via the cli.

To be consistent in ci & in our local development experience, we also overwrote the rustfmt command for rust-analyzer in both vscode and helix.
If you are using one of these editors you will need to install rustfmt-unstable via `cargo install rustfmt-unstable` to get formatting working.
