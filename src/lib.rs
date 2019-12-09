//!
//! Logger middleware for Iron framework, with slog-rs
//!

#![deny(missing_docs, warnings)]

#[macro_use]
extern crate slog;
extern crate chrono;
extern crate iron;

mod format;
pub use format::{LogContext, LogFormatter, DefaultLogFormatter};

use std::fmt;
use iron::{Request, Response, IronResult, Handler};
use slog::Logger;


struct Format<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e, 'f, F: ?Sized + LogFormatter> {
    context: LogContext<'req, 'res, 'a, 'b, 's, 'e>,
    formatter: &'f F,
}

impl<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e, 'f, F: ?Sized + LogFormatter> fmt::Display
    for Format<'req, 'res, 'a, 'b, 's, 'e, 'f, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter.format(f, &self.context)
    }
}


/// Middleware for logging information of request and response, to certain logger.
pub struct LoggerMiddleware<H: Handler, F: LogFormatter> {
    formatter: F,
    handler: H,
    logger: Logger,
}

impl<H: Handler, F: LogFormatter> LoggerMiddleware<H, F> {
    /// Create a `LoggerMiddleware` middleware with specified `logger` and `formatter`.
    ///
    /// ```ignore
    /// let handler = create_your_handler();
    /// let logger: slog::Logger = create_your_logger();
    /// let formatter = DefaultLogFormatter;
    /// let logged_handler = LoggerMiddleware::new(handler, logger, formatter);
    /// ```
    pub fn new(handler: H, logger: Logger, formatter: F) -> Self {
        LoggerMiddleware {
            handler,
            logger,
            formatter,
        }
    }
}

impl<H: Handler, F: LogFormatter> Handler for LoggerMiddleware<H, F> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let start_time = chrono::Local::now();
        let result = self.handler.handle(req);
        let end_time = chrono::Local::now();

        match result {
            Ok(res) => {
                info!(self.logger,
                      "{}",
                      Format {
                          context: LogContext {
                              req,
                              res: &res,
                              start_time: &start_time,
                              end_time: &end_time,
                          },
                          formatter: &self.formatter,
                      });
                Ok(res)
            }
            Err(err) => {
                error!(self.logger,
                       "{}",
                       Format {
                           context: LogContext {
                               req,
                               res: &err.response,
                               start_time: &start_time,
                               end_time: &end_time,
                           },
                           formatter: &self.formatter,
                       });
                Err(err)
            }
        }
    }
}
