use iron::{Request, Response};
use std::fmt;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct LogContext<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e> {
    pub req: &'req Request<'a, 'b>,
    pub res: &'res Response,
    pub start_time: &'s DateTime<Local>,
    pub end_time: &'e DateTime<Local>,
}


pub trait LogFormatter: 'static + Send + Sync {
    fn format(&self, f: &mut fmt::Formatter, context: &LogContext) -> fmt::Result;
}

impl<F> LogFormatter for F
    where F: 'static + Send + Sync + Fn(&mut fmt::Formatter, &LogContext) -> fmt::Result
{
    fn format(&self, f: &mut fmt::Formatter, context: &LogContext) -> fmt::Result {
        (*self)(f, context)
    }
}


pub struct DefaultLogFormatter;

impl LogFormatter for DefaultLogFormatter {
    fn format(&self, f: &mut fmt::Formatter, ctx: &LogContext) -> fmt::Result {
        write!(f, "{} ", ctx.req.method)?;
        write!(f, "{} ", ctx.req.url)?;
        match ctx.res.status {
            Some(status) => write!(f, "{} ", status)?,
            None => write!(f, "<missing status code>")?,
        }
        let response_time = calc_elapsed_ms(&ctx.start_time, &ctx.end_time);
        write!(f, "({} ms)", response_time)?;
        Ok(())
    }
}


fn timestamp_msec(t: &DateTime<Local>) -> f64 {
    t.timestamp() as f64 * 1000f64 + t.timestamp_subsec_millis() as f64
}

fn calc_elapsed_ms(start: &DateTime<Local>, end: &DateTime<Local>) -> f64 {
    let start_timestamp = timestamp_msec(start);
    let end_timestamp = timestamp_msec(end);
    end_timestamp - start_timestamp
}
