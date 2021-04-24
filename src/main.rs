use log::{debug, error, info, trace, warn};
use log4rs;
use std::env;
use std::path::Path;
use std::process;

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

    foo::bar::baz::test_logging();

    fee::fi::test_logging();

    fee::fi::fo::fum::test_logging();
}

/// The `foo` module contains submodules with their own logging functions.
pub mod foo {
    /// The `bar` module contains a submodule with its own logging function.
    pub mod bar {
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
}

mod fee {
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

        pub mod fo {
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
