//! # A Beginner's Guide to Rust
//!
//! This Rust crate provides a tutorial for using
//! [log4rs](https://docs.rs/log4rs), which according to its documentation "is a
//! highly configurable logging framework modeled after Java's Logback and log4j
//! libraries".
//!
//! The log4rs crate does provide its own documentation.  However, that
//! documentation is written from an architectural and implementation-oriented
//! point-of-view, and it can be hard to quickly scan it to find out how to do
//! what you want to do.  In contrast, this tutorial has a _declarative_ focus,
//! meaning it is based on desired results rather than on the details of how it
//! all actually works internally.  We start with a set of small, minimal
//! examples, then gradually build on those to introduce additional features.
//!
//! You can think of this as a set of snippets or recipes you can copy, allowing
//! you to take advantage of log4rs without studying all the documentation and
//! code.
//!
//! It is assumed that the reader has a basic understanding of how to write Rust
//! code and how to use the [Cargo](https://doc.rust-lang.org/cargo/).  Some
//! prior knowledge of [YAML](https://yaml.org) would also be useful.
//!
//! The documentation in this module describes the overall organization of the
//! modules and files in this workspace, and provides instructions for initial
//! setup of a project that will use log4rs.
//!
//! ## Cargo Setup
//!
//! To use the log4rs crate in your workspace, you must add it to the
//! `[dependencies]` section of your `Cargo.toml` file.  You also need to use
//! Rust's standard _logging facade_ crate, [log](https://crates.io/crates/log).
//! So, your `Cargo.toml` should contain these lines:
//!
//! ```toml
//! [dependencies]
//! log = 0.4
//! log4rs = 1.0
//! ```
//!
//! ## Configuration Files
//!
//! log4rs is designed to be easily configurable via a _configuration file_
//! which is read by your program at startup.  Use of a configuration file makes
//! it easy to modify the logging behavior without any need to change or
//! recompile code.
//!
//! The most common format for log4rs configuration files is YAML, but JSON and
//! TOML are also supported.  Most of the examples we present use YAML, but
//! there are some examples of the other formats.
//!
//! To load a configuration file, call the
//! [log4rs::init_file()](https://docs.rs/log4rs/1.0.0/log4rs/fn.init_file.html)
//! function.  If there are any errors in the configuration, then error messages
//! will be printed to standard error and the function will return an error.
//!
//! The code to use it will be something like this:
//!
//! ```rust
//! use log4rs;
//!
//! fn main() {
//!     // Initialize logger
//!     log4rs::init_file("my_config.yaml", Default::default()).unwrap_or_else(|e| {
//!         eprintln!("error: unable to initialize logger: {}", e);
//!         std::process::abort();
//!     });
//!
//!     // Continue with the rest of the program...
//! }
//! ```
//!
//! The rest of this tutorial will cover what to put in that `my_config.yaml`
//! file.
//!
//! Note that instead of using a configuration file, you can also write code to
//! configure log4rs.  That is outside the scope of this tutorial; see the
//! log4rs documentation for details of how to configure it programmatically.
//!
//! ## Generating Log Messages
//!
//! The [log](https://docs.rs/log/) crate provides a simple logging API that is
//! shared by many logging implementations, including log4rs.  The `error!()`,
//! `warn!()`, `info!()`, `debug!()`, and `trace!()` macros can be used to
//! generate a log entry.  Using them is as simple as this:
//!
//! ```rust
//! use log::{debug, error, info, trace, warn};
//!
//! // ...
//!
//!     info!("This is an info message.");
//!     error!("This is an error message.");
//!     warn!("This is a warning message.");
//!     debug!("This is a debug message.");
//!     trace!("This is a trace message.");
//!
//! // ...
//! ```
//!
//! Each log entry has a _target_, which by default is the module name.
//! However, you can specify an different target by using the `target:`
//! argument, like this:
//!
//! ```rust
//! // ...
//!
//!     debug!(target: "fileio", "This is a debug message.");
//!
//! // ...
//! ```
//!
//! Each log entry call will automatically capture the current date and time,
//! source code file and line, module, target, and other metadata in a _log
//! record_ and then pass that to the configured logger implementation.  The
//! logger is responsible for formatting a log message and writing it in
//! whatever output medium is appropriate.
//!
//! ## On To The Examples and Tutorial
//!
//! This crate contains a simple executable that loads a configuration file and
//! then generates a series of log messages, so you can see the results.  To run
//! it, just specify the configuration filename as an argument to `cargo run`,
//! like this:
//!
//! ```bash
//! cargo run -- minimal_stdout_with_trace.yaml
//! ```
//!
//! Of course, you can use this to test configuration files you've written
//! yourself, not just the provided configurations.
//!
//! The tutorial is in the [docs] submodule.  You can go there now to start.

use log::{debug, error, info, trace, warn};
use log4rs;
use std::env;
use std::path::Path;
use std::process;

pub mod docs;

/// The `main()` function reads an optional command-line argument, initializes
/// log4rs using the specified file (which defaults to `minimal_stdout.yaml` if
/// not specified), and then logs a series of messages.
fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = match args.len() {
        1 => "minimal_stdout.yaml",
        2 => args[1].as_ref(),
        _ => {
            eprintln!("error: program accepts only one command-line argument");
            process::abort();
        }
    };

    if !Path::new(config_path).exists() {
        eprintln!("error: \"{}\" does not exist", config_path);
        process::abort();
    }

    log4rs::init_file(config_path, Default::default()).unwrap_or_else(|e| {
        eprintln!("error: unable to initialize logger: {}", e);
        process::abort();
    });

    eprintln!("using configuration file {}:", config_path);
    test_logging()
}

/// Generates a series of log messages from the top-level module, then generates
/// log messages from submodules.
fn test_logging() {
    info!("This is an info message.");
    error!("This is an error message.");
    warn!("This is a warning message.");
    debug!("This is a debug message.");
    trace!("This is a trace message.");

    foo::bar::test_logging();

    foo::baz::test_logging();

    fee::fi::test_logging();

    fee::fi::fo::fum::test_logging();
}

/// The `foo` module contains submodules with their own logging functions.
pub mod foo {
    /// The `bar` module contains a submodule with its own logging function.
    pub mod bar {
        use log::*;

        /// Generates a series of "This is a message from bar." log messages
        /// for all logging levels.
        pub fn test_logging() {
            info!("This is an info message from bar.");
            error!("This is an error message from bar.");
            warn!("This is a warning message from bar.");
            debug!("This is a debug message from bar.");
            trace!("This is a trace message from bar.");
        }
    }

    /// The `baz` module contains a logging function.
    pub mod baz {
        use log::*;

        /// Generates a series of "This is a message from baz." log messages
        /// for all logging levels.
        pub fn test_logging() {
            info!("This is an info message from baz.");
            error!("This is an error message from baz.");
            warn!("This is a warning message from baz.");
            debug!("This is a debug message from baz.");
            trace!("This is a trace message from baz.");
        }
    }
}

/// The `fee` module contains submodules with their own logging functions.
mod fee {
    /// The `fi` module contains a `test_logging()` function, and submodules
    /// with their own logging functions.
    pub mod fi {
        use log::*;

        /// Generates a series of "This is a message from fie." log
        /// messages for all logging levels.
        pub fn test_logging() {
            info!("This is an info message from fi.");
            error!("This is an error message from fi.");
            warn!("This is a warning message from fi.");
            debug!("This is a debug message from fi.");
            trace!("This is a trace message from fi.");
        }

        /// The `fo` module contains submodules with their own logging
        /// functions.
        pub mod fo {
            /// The `fum` module contains a `test_logging()` function.
            pub mod fum {
                use log::*;

                /// Generates a series of "This is a message from fum." log
                /// messages for all logging levels.
                pub fn test_logging() {
                    info!("This is an info message from fum.");
                    error!("This is an error message from fum.");
                    warn!("This is a warning message from fum.");
                    debug!("This is a debug message from fum.");
                    trace!("This is a trace message from fum.");
                }
            }
        }
    }
}
