extern crate iron_slog;
extern crate iron;
extern crate router;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::{Drain, Logger};
use iron::{Iron, Request, Response, IronResult, status};
use router::Router;
use iron_slog::{LoggerMiddleware, DefaultLogFormatter};

fn hello(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello")))
}

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());

    let formatter = DefaultLogFormatter;
    // To use custom format:
    //
    // use iron_slog::LogContext;
    // fn formatter(f: &mut std::fmt::Formatter, context: &LogContext) -> std::fmt::Result {
    //     write!(f, "{:?}", context)
    // }

    let mut router = Router::new();
    router.get("/", hello, "hello");

    let handler = LoggerMiddleware::new(router, logger, formatter);

    Iron::new(handler).http("0.0.0.0:3000").unwrap();
}
