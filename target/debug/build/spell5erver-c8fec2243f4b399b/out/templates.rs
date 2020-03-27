pub mod templates {
use std::io::{self, Write};
use std::fmt::Display;

pub mod statics;mod template_spell_card_html;
pub use self::template_spell_card_html::spell_card_html;

#[deprecated(since="0.7.4", note="please use `spell_card_html` instead")]
pub use self::spell_card_html as spell_card;

/// This trait should be implemented for any value that can be the
/// result of an expression in a template.
///
/// This trait decides how to format the given object as html.
/// There exists a default implementation for any `T: Display` that
/// formats the value using Display and then html-encodes the result.
pub trait ToHtml {
    /// Write self to `out`, which is in html representation.
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()>;
}

/// Wrapper object for data that should be outputted as raw html
/// (objects that may contain markup).
#[allow(dead_code)]
pub struct Html<T>(pub T);

impl<T: Display> ToHtml for Html<T> {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", self.0)
    }
}

impl<T: Display> ToHtml for T {
    #[inline]
    fn to_html(&self, out: &mut dyn Write) -> io::Result<()> {
        write!(ToHtmlEscapingWriter(out), "{}", self)
    }
}

struct ToHtmlEscapingWriter<'a>(&'a mut dyn Write);

impl<'a> Write for ToHtmlEscapingWriter<'a> {
    #[inline]
    // This takes advantage of the fact that `write` doesn't have to write everything,
    // and the call will be retried with the rest of the data
    // (it is a part of `write_all`'s loop or similar.)
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        // quickly skip over data that doesn't need escaping
        let n = data
            .iter()
            .take_while(|&&c| {
                c != b'"' && c != b'&' && c != b'\'' && c != b'<' && c != b'>'
            })
            .count();
        if n > 0 {
            self.0.write(&data[0..n])
        } else {
            Self::write_one_byte_escaped(&mut self.0, data)
        }
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl<'a> ToHtmlEscapingWriter<'a> {
    #[inline(never)]
    fn write_one_byte_escaped(
        out: &mut impl Write,
        data: &[u8],
    ) -> io::Result<usize> {
        let next = data.get(0);
        out.write_all(match next {
            Some(b'"') => b"&quot;",
            Some(b'&') => b"&amp;",
            Some(b'<') => b"&lt;",
            Some(b'>') => b"&gt;",
            None => return Ok(0),
            // we know this function is called only for chars that need escaping,
            // so we don't have to handle the "other" case (this one is for `'`)
            _ => b"&#39;",
        })?;
        Ok(1)
    }
}
use mime::TEXT_HTML_UTF_8;
use warp::http::{header::CONTENT_TYPE, response::Builder};
use warp::{reject::custom, reject::Reject, Rejection, reply::Response};

/// Extension trait for [`response::Builder`] to simplify template rendering.
///
/// Render a template to a buffer, and use that buffer to complete a
/// `Response` from the builder.  Also set the content type of the
/// response to `TEXT_HTML_UTF_8`.
///
/// # Examples
///
/// Give a template `page`, that takes two arguments other than the
/// `Write` buffer, this will use the variables `title` and `body` and
/// render the template to a response.
///
/// ```
/// # extern crate warp;
/// # use std::io::{self, Write};
/// # use warp::http::Response;
/// # use ructe::templates::RenderRucte;
/// # fn page(o: &mut Write, _: u8, _: u8) -> io::Result<()> { Ok(()) }
/// # let (title, body) = (47, 11);
/// Response::builder().html(|o| page(o, title, body))
/// # ;
/// ```
///
/// Other builder methods can be called before calling the `html` method.
/// Here is an example that sets a cookie in the Response.
///
/// ```
/// # extern crate warp;
/// # use std::io::{self, Write};
/// # use warp::http::{header::SET_COOKIE, Response};
/// # use ructe::templates::RenderRucte;
/// # fn page(o: &mut Write, _: u8, _: u8) -> io::Result<()> { Ok(()) }
/// # let (title, body, value) = (47, 11, 14);
/// Response::builder()
///     .header(SET_COOKIE, format!("FOO={}, SameSite=Strict; HttpOnly", value))
///     .html(|o| page(o, title, body))
/// # ;
/// ```
///
/// [`response::Builder`]: ../../http/response/struct.Builder.html
pub trait RenderRucte {
    /// Render a template on the response builder.
    ///
    /// This is the main function of the trait.  Please see the trait documentation.
    fn html<F>(self, f: F) -> Result<Response, Rejection>
    where
        F: FnOnce(&mut Vec<u8>) -> io::Result<()>;
}

impl RenderRucte for Builder {
    fn html<F>(self, f: F) -> Result<Response, Rejection>
    where
        F: FnOnce(&mut Vec<u8>) -> io::Result<()>,
    {
        let mut buf = Vec::new();
        f(&mut buf).map_err(RenderError::Write).map_err(custom)?;
        self.header(CONTENT_TYPE, TEXT_HTML_UTF_8.as_ref())
            .body(buf.into())
            .map_err(RenderError::Build)
            .map_err(custom)
    }
}

#[derive(Debug)]
enum RenderError {
    Write(std::io::Error),
    Build(warp::http::Error),
}

impl Reject for RenderError {}

}
