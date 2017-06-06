#[macro_use]
extern crate slog;
extern crate chrono;
extern crate iron;

mod format;
pub use format::{LogContext, LogFormatter, DefaultLogFormatter};

use std::fmt;
use iron::{Request, Response, IronResult, Handler};
use slog::Logger;
use chrono::{DateTime, Local};


struct Format<'req, 'res, 'a: 'req, 'b: 'a, 'f, F: ?Sized + LogFormatter> {
    req: &'req Request<'a, 'b>,
    res: &'res Response,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    f: &'f F,
}

impl<'req, 'res, 'a: 'req, 'b: 'a, 'f, F: LogFormatter> fmt::Display for Format<'req, 'res, 'a, 'b, 'f, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let context = LogContext {
            req: &self.req,
            res: &self.res,
            start_time: &self.start_time,
            end_time: &self.end_time,
        };
        self.f.format(f, &context)
    }
}


pub struct LoggerMiddleware<H: Handler, F: LogFormatter> {
    formatter: F,
    handler: H,
    logger: Logger,
}

impl<H: Handler, F: LogFormatter> LoggerMiddleware<H, F> {
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
                {
                    let f = Format {
                        req,
                        res: &res,
                        start_time,
                        end_time,
                        f: &self.formatter,
                    };
                    info!(self.logger, "{}", f);
                }
                Ok(res)
            }
            Err(err) => {
                {
                    let f = Format {
                        req,
                        res: &err.response,
                        start_time,
                        end_time,
                        f: &self.formatter,
                    };
                    error!(self.logger, "{}", f);
                }
                Err(err)
            }
        }
    }
}
