#[macro_use]
extern crate slog;
extern crate chrono;
extern crate iron;

use iron::{Request, Response, IronResult, IronError, Handler, typemap};
use slog::Logger;
use chrono::DateTime;


pub struct LoggerMiddleware<H: Handler> {
    handler: H,
    logger: Logger,
}

impl<H: Handler> LoggerMiddleware<H> {
    pub fn new(handler: H, logger: Logger) -> Self {
        LoggerMiddleware { handler, logger }
    }

    fn log(&self, start_time: DateTime<chrono::Local>, req: &mut Request, res: &Response) {
        let end_time: DateTime<chrono::Local> = chrono::Local::now();
        fn timestamp_msec(t: &chrono::DateTime<chrono::Local>) -> f64 {
            t.timestamp() as f64 * 1000f64 + t.timestamp_subsec_millis() as f64
        }
        let start_timestamp = timestamp_msec(&start_time);
        let end_timestamp = timestamp_msec(&end_time);
        let response_time: f64 = end_timestamp - start_timestamp;

        // formatting
        let method = format!("{}", req.method);
        let url = format!("{}", req.url);
        let status = res.status
                        .map(|s| format!("{}", s))
                        .unwrap_or("<missing status code>".to_string());
        let response_time = format!("{} ms", response_time);
        let remote_addr = format!("{}", req.remote_addr);
        let request_time = format!("{}", start_time.format("%Y-%m-%dT%H:%M:%S.%fZ%z"));
        info!(self.logger,
              "{} {} {} ({}), {} {}",
              method,
              url,
              status,
              response_time,
              remote_addr,
              request_time);
    }
}

impl<H: Handler> Handler for LoggerMiddleware<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let start_time = chrono::Local::now();
        match self.handler.handle(req) {
            Ok(res) => {
                self.log(start_time, req, &res);
                Ok(res)
            }
            Err(err) => {
                self.log(start_time, req, &err.response);
                Err(err)
            }
        }
    }
}
