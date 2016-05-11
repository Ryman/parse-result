#![warn(missing_docs)]

//! Add [`parse`](./trait.ParseResult.html#tymethod.parse) to `Result`
//!
//! The core purpose of this library is to give better ergonomics to parsing from
//! sources which could fail, e.g. reading `stdin`.
//!
//! ```no_run
//! extern crate parse_result;
//! use parse_result::*;
//! use std::env;
//!
//! fn main() {
//!     // It turns code like this:
//!     env::var("PORT").map(|s| s.parse().unwrap_or(3000)).unwrap_or(3000);
//!
//!     // Into this:
//!     env::var("PORT").parse().unwrap_or(3000);
//!
//!     // Matching to find the specific failure
//!     match env::var("PORT").parse::<u32>() {
//!         Ok(port) => println!("Parsed port {} successfully!", port),
//!         Err(OriginalErr(e)) => panic!("Failed to get PORT from env: {}", e),
//!         Err(ParseFailure(e)) => panic!("Failed to parse PORT: {}", e),
//!     }
//! }
//! ```

use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::str::FromStr;

pub use Error::*;

#[doc(inline)]
/// Extension trait to add `parse` to `Result`.
pub trait ParseResult<E> {
    /// Parses the `Result` into another type if it's not already an `Err` value.
    ///
    /// See [`str::parse`](http://doc.rust-lang.org/std/primitive.str.html#method.parse) for
    /// more information and examples.
    fn parse<F>(self) -> Result<F, Error<E, F::Err>>
    where F: FromStr;
}

/// Represents the possible errors from calling `parse` on `Result`.
#[derive(Debug, PartialEq, Eq)]
pub enum Error<E, P> {
    /// A pre-existing `Err` from before attempting to parse.
    OriginalErr(E),

    /// An `Err` generated as a result from parsing.
    ParseFailure(P)
}

impl<T, E> ParseResult<E> for Result<T, E>
where T: AsRef<str> {
    fn parse<F>(self) -> Result<F, Error<E, F::Err>>
    where F: FromStr {
        self.map_err(OriginalErr)
            .and_then(|s| s.as_ref().parse().map_err(ParseFailure))
    }
}

impl<E, P> StdError for Error<E, P>
where E: StdError, P: StdError {
    fn description(&self) -> &str {
        match *self {
            OriginalErr(ref e) => e.description(),
            ParseFailure(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            OriginalErr(ref e) => Some(e),
            ParseFailure(ref e) => Some(e),
        }
    }
}

impl<E, P> Display for Error<E, P>
where E: Display, P: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OriginalErr(ref e) => e.fmt(f),
            ParseFailure(ref e) => e.fmt(f),
        }
    }
}

#[test]
fn parses_ok_with_type_inference() {
    let val: Result<&str, ()> = Ok("42");

    assert_eq!(val.parse(), Ok(42));
}

#[test]
fn allows_turbofish_usage() {
    use std::any::Any;
    use std::net::{IpAddr, AddrParseError};

    let val: Result<&str, &str> = Ok("42");

    if let Err(ParseFailure(err)) = val.parse::<IpAddr>() {
        assert!(Any::is::<AddrParseError>(&err));
    } else {
        panic!("Should have failed to parse as an IpAddr");
    }

    assert_eq!(val.parse::<u32>(), Ok(42));
    assert_eq!(val.parse::<i64>(), Ok(42));
}

#[test]
fn fails_to_parse_an_original_err() {
    let val: Result<&str, &str> = Err("Failed to load data");

    assert_eq!(val.parse::<i32>(), Err(OriginalErr("Failed to load data")));
}

#[test]
fn returns_parse_error_on_parse_failure() {
    use std::any::Any;
    use std::num::ParseIntError;

    let val: Result<&str, &str> = Ok("hello");

    if let Err(ParseFailure(err)) = val.parse::<i32>() {
        assert!(Any::is::<ParseIntError>(&err));
    } else {
        panic!("Should have failed to parse as an i32");
    }
}

#[test]
fn boxed_error_works() {
    use std::env;
    use std::error::Error;

    fn get_port() -> Result<u16, Box<Error>> {
        Ok(try!(env::var("PORT").parse()))
    }

    assert!(get_port().is_err())
}
