# A Guide to Using log4rs

This repository contains a beginner's guide to using the
[log4rs](https://github.com/estk/log4rs) logging framework for
[Rust](https://www.rust-lang.org/).

* log4rs Documentation: <https://docs.rs/log4rs>
* log4rs Repository: <https://github.com/estk/log4rs>

This repository contains all the documentation, example configuration files, and
other source code needed to build the documentation and run the examples.

To generate the documentation and display it in a web browser:

    cargo doc --no-deps --open

To run one of the examples, or test a configuration file you've created
yourself, pass the name of the YAML file to `cargo run --`.  For
example:

    cargo run -- minimal_stdout_with_trace.yaml
