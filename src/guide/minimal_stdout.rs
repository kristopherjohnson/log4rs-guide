//! # Minimal Standard Output Logging
//!
//! We will start with the simplest useful configuration, taking advantage of
//! all the default behavior we can.  Two elements are _required_: at least one
//! _appender_, and definition of the _root_ logger.
//!
//! You can think of an _appender_ as a "destination", such as standard output,
//! or standerd error, or a log file.  We will use a simple
//! [console](https://docs.rs/log4rs/1.0.0/log4rs/append/console/struct.ConsoleAppenderDeserializer.html#configuration)
//! appender named "log" that writes to standard output.
//!
//! We also need to configure the _root_ logger.  This is the top of a
//! hierarchical tree of logger objects.  We will build out more complicated
//! hierarchies later in the tutorial, but for now we will just define the root
//! so that it sends everything to the "log" appender.
//!
//! Our minimal configuration file looks like this:
//!
//! ```not_run
//! appenders:
//!   log:
//!     kind: console
//!
//! root:
//!   appenders:
//!     - log
//! ```
//!
//! When we run our test program with this configuration, the output looks like this:
//!
//! ```not_run
//! 2021-04-27T19:37:27.190275-04:00 INFO log4rs_guide - This is an info message.
//! 2021-04-27T19:37:27.190999-04:00 ERROR log4rs_guide - This is an error message.
//! 2021-04-27T19:37:27.191032-04:00 WARN log4rs_guide - This is a warning message.
//! 2021-04-27T19:37:27.191061-04:00 DEBUG log4rs_guide - This is a debug message.
//! 2021-04-27T19:37:27.191095-04:00 INFO log4rs_guide::foo::bar - This is an info message from bar.
//! 2021-04-27T19:37:27.191125-04:00 ERROR log4rs_guide::foo::bar - This is an error message from bar.
//! 2021-04-27T19:37:27.191153-04:00 WARN log4rs_guide::foo::bar - This is a warning message from bar.
//! 2021-04-27T19:37:27.191182-04:00 DEBUG log4rs_guide::foo::bar - This is a debug message from bar.
//! 2021-04-27T19:37:27.191210-04:00 INFO log4rs_guide::foo::baz - This is an info message from baz.
//! ...
//! ```
//!
//! These are the fields of a log entry, using the default format:
//!
//! ```not_run
//! 2021-04-27T19:37:27.191095-04:00 INFO log4rs_guide::foo::bar - This is an info message from bar.
//! \                              / \  / \                    /   \                               /
//!  ------------------------------   --   --------------------     -------------------------------
//!               time               level       target                        message
//! ```
//!
//! We can see INFO, ERROR, WARN, DEBUG, and INFO messages here, but we don't
//! see any TRACE-level messages, which our test program does generate.  This is
//! because by default the root logger does not show trace messages.  We will
//! see how to enable that in the next topic.
//!
//! Next: [minimal_stdout_with_trace](super::minimal_stdout_with_trace)
//!
