//! ## Minimal Standard Output with Trace
//!
//! In [the previous module](super::minimal_stdout), we had a configuration that
//! wrote to standard output but skipped trace-level messages.  This is because
//! by default, the `root` logger only accepts messages of _debug_ level and
//! lower.  To enable trace messages, we have to set the `level` of the `root`
//! element to `trace`, like this:
//!
//! ```not_run
//! appenders:
//!   log:
//!     kind: console
//!
//! root:
//!   level: trace
//!   appenders:
//!     - log
//! ```
//!
//! Now when we run our test program with this configuration, we see trace
//! messages along with the rest.
//!
//! ```not_run
//! 2021-04-27T20:02:20.084546-04:00 INFO log4rs_guide - This is an info message.
//! 2021-04-27T20:02:20.085296-04:00 ERROR log4rs_guide - This is an error message.
//! 2021-04-27T20:02:20.085330-04:00 WARN log4rs_guide - This is a warning message.
//! 2021-04-27T20:02:20.085359-04:00 DEBUG log4rs_guide - This is a debug message.
//! 2021-04-27T20:02:20.085387-04:00 TRACE log4rs_guide - This is a trace message.
//! 2021-04-27T20:02:20.085416-04:00 INFO log4rs_guide::foo::bar - This is an info message from bar.
//! 2021-04-27T20:02:20.085445-04:00 ERROR log4rs_guide::foo::bar - This is an error message from bar.
//! 2021-04-27T20:02:20.085474-04:00 WARN log4rs_guide::foo::bar - This is a warning message from bar.
//! 2021-04-27T20:02:20.085503-04:00 DEBUG log4rs_guide::foo::bar - This is a debug message from bar.
//! 2021-04-27T20:02:20.085540-04:00 TRACE log4rs_guide::foo::bar - This is a trace message from bar.
//! 2021-04-27T20:02:20.085569-04:00 INFO log4rs_guide::foo::baz - This is an info message from baz.
//! ```
//!
//! You can also set the root level to reduce the amount of output.  For
//! example, if you set `level: error` on the root element, then you will see
//! only _info_ and _error_ messages in the output.
//!
//! Next: TODO
