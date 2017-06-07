use iron::{Request, Response};
use std::fmt;
use chrono::{DateTime, Duration, Local};

/// Information of request/response for logging
#[derive(Debug)]
pub struct LogContext<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e> {
    /// `Request`
    pub req: &'req Request<'a, 'b>,
    /// `Response`
    pub res: &'res Response,
    /// start time
    pub start_time: &'s DateTime<Local>,
    /// end time
    pub end_time: &'e DateTime<Local>,
}

impl<'req, 'res, 'a: 'req, 'b: 'a, 's, 'e> LogContext<'req, 'res, 'a, 'b, 's, 'e> {
    /// Calculate response time
    pub fn response_time(&self) -> Duration {
        self.end_time.signed_duration_since(self.start_time.clone())
    }
}


/// Formatter.
pub trait LogFormatter: 'static + Send + Sync {
    /// Write `context` to `f`.
    fn format(&self, f: &mut fmt::Formatter, context: &LogContext) -> fmt::Result;
}

impl<F> LogFormatter for F
    where F: 'static + Send + Sync + Fn(&mut fmt::Formatter, &LogContext) -> fmt::Result
{
    fn format(&self, f: &mut fmt::Formatter, context: &LogContext) -> fmt::Result {
        (*self)(f, context)
    }
}


/// Default formatter.
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
