use iron::{Request, Response};
use std::fmt;
use chrono::{DateTime, Duration, Local};

#[derive(Debug)]
pub struct LogContext<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e> {
    pub req: &'req Request<'a, 'b>,
    pub res: &'res Response,
    pub start_time: &'s DateTime<Local>,
    pub end_time: &'e DateTime<Local>,
}

impl<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e> LogContext<'req, 'res, 'a, 'b, 's, 'e> {
    pub fn response_time(&self) -> Duration {
        self.end_time.signed_duration_since(self.start_time.clone())
    }
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
        write!(f, "({} ms)", ctx.response_time().num_milliseconds())?;
        Ok(())
    }
}
