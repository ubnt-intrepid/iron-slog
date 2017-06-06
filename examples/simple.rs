extern crate iron_slog;
extern crate iron;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use std::io;
use slog::{Drain, Logger};
use iron::{Iron, Request, Response, IronResult, status};
use iron_slog::LoggerMiddleware;

fn main() {
    let decorator = slog_term::PlainDecorator::new(io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());

    fn handler(_req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello")))
    }

    let logged_handler = LoggerMiddleware::new(handler, logger);

    Iron::new(logged_handler).http("0.0.0.0:3000").unwrap();
}
