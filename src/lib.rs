#[macro_use]
extern crate slog;
extern crate chrono;
extern crate iron;

use std::fmt;
use iron::{Request, Response, IronResult, Handler};
use slog::Logger;
use chrono::{DateTime, Local};


fn timestamp_msec(t: &chrono::DateTime<chrono::Local>) -> f64 {
    t.timestamp() as f64 * 1000f64 + t.timestamp_subsec_millis() as f64
}

fn calc_elapsed_ms(start: &DateTime<Local>, end: &DateTime<Local>) -> f64 {
    let start_timestamp = timestamp_msec(start);
    let end_timestamp = timestamp_msec(end);
    end_timestamp - start_timestamp
}


struct FormatContext<'req, 'res, 'a: 'req, 'b: 'a> {
    req: &'req Request<'a, 'b>,
    res: &'res Response,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
}

impl<'req, 'res, 'a: 'req, 'b: 'a> fmt::Display for FormatContext<'req, 'res, 'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let response_time = calc_elapsed_ms(&self.start_time, &self.end_time);
        let status = self.res
                         .status
                         .map(|s| format!("{}", s))
                         .unwrap_or("<missing status code>".to_string());
        write!(f,
               "{} {} {} ms ({}), {} {}",
               self.req.method,
               self.req.url,
               status,
               response_time,
               self.req.remote_addr,
               self.start_time.format("%Y-%m-%dT%H:%M:%S.%fZ%z"))
    }
}


pub struct LoggerMiddleware<H: Handler> {
    handler: H,
    logger: Logger,
}

impl<H: Handler> LoggerMiddleware<H> {
    pub fn new(handler: H, logger: Logger) -> Self {
        LoggerMiddleware { handler, logger }
    }

    fn log(&self, start_time: DateTime<Local>, end_time: DateTime<Local>, req: &mut Request, res: &Response) {
        let context = FormatContext {
            req,
            res,
            start_time,
            end_time,
        };
        info!(self.logger, "{}", context);
    }
}

impl<H: Handler> Handler for LoggerMiddleware<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let start_time = chrono::Local::now();
        let result = self.handler.handle(req);
        let end_time = chrono::Local::now();

        match result {
            Ok(res) => {
                self.log(start_time, end_time, req, &res);
                Ok(res)
            }
            Err(err) => {
                self.log(start_time, end_time, req, &err.response);
                Err(err)
            }
        }
    }
}
