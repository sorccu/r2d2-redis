//! Redis support for the `r2d2` connection pool.
#![doc(html_root_url = "https://docs.rs/r2d2_redis/0.12.0")]
#![warn(missing_docs)]

pub extern crate r2d2;
pub extern crate redis;

use std::error;
use std::error::Error as _StdError;
use std::fmt;

/// A unified enum of errors returned by redis::Client
#[derive(Debug)]
pub enum Error {
    /// A redis::RedisError
    Other(redis::RedisError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
        match self.cause() {
            Some(cause) => write!(fmt, "{}: {}", self.description(), cause),
            None => write!(fmt, "{}", self.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Other(ref err) => {
                #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
                err.cause()
            },
        }
    }
}

/// An `r2d2::ConnectionManager` for `redis::Client`s.
///
/// ## Example
///

/// ```
/// extern crate r2d2_redis;
///
/// use std::ops::DerefMut;
/// use std::thread;
///
/// use r2d2_redis::{r2d2, redis, RedisConnectionManager};
///
/// fn main() {
///     let manager = RedisConnectionManager::new("redis://localhost").unwrap();
///     let pool = r2d2::Pool::builder()
///         .build(manager)
///         .unwrap();
///
///     let mut handles = vec![];
///
///     for _i in 0..10i32 {
///         let pool = pool.clone();
///         handles.push(thread::spawn(move || {
///             let mut conn = pool.get().unwrap();
///             let reply = redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
///             // Alternatively, without deref():
///             // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
///             assert_eq!("PONG", reply);
///         }));
///     }
///
///     for h in handles {
///         h.join().unwrap();
///     }
/// }
/// ```
#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: redis::ConnectionInfo,
}

impl RedisConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See `redis::Client::open` for a description of the parameter
    /// types.
    pub fn new<T: redis::IntoConnectionInfo>(
        params: T,
    ) -> Result<RedisConnectionManager, redis::RedisError> {
        Ok(RedisConnectionManager {
            connection_info: params.into_connection_info()?,
        })
    }
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = Error;

    fn connect(&self) -> Result<redis::Connection, Error> {
        match redis::Client::open(self.connection_info.clone()) {
            Ok(client) => client.get_connection().map_err(Error::Other),
            Err(err) => Err(Error::Other(err)),
        }
    }

    fn is_valid(&self, conn: &mut redis::Connection) -> Result<(), Error> {
        redis::cmd("PING").query(conn).map_err(Error::Other)
    }

    fn has_broken(&self, conn: &mut redis::Connection) -> bool {
        !conn.is_open()
    }
}
