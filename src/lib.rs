//! Redis support for the `r2d2` connection pool.
#![doc(html_root_url="https://nevdelap.github.io/r2d2-redis/doc")]
#![warn(missing_docs)]
extern crate r2d2;
extern crate redis;

use std::error;
use std::error::Error as _StdError;
use std::fmt;
use std::ops::Deref;

/// A unified enum of errors returned by redis::Client
#[derive(Debug)]
pub enum Error {
    /// A redis::RedisError
    Other(redis::RedisError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.description(), self.cause().unwrap())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Other(ref err) => err.cause()
        }
    }
}

/// An `r2d2::ConnectionManager` for `redis::Client`s.
///
/// ## Example
///

/// ```
/// extern crate r2d2;
/// extern crate r2d2_redis;
/// extern crate redis;
///
/// use std::default::Default;
/// use std::ops::Deref;
/// use std::sync::Arc;
/// use std::thread;
/// use r2d2_redis::RedisConnectionManager;
///
/// fn main() {
///     let config = Default::default();
///     let manager = RedisConnectionManager::new("redis://localhost").unwrap();
///     let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
///
///     let mut handles = vec![];
///
///     for _i in 0..10i32 {
///         let pool = pool.clone();
///         handles.push(thread::spawn(move || {
///             let conn = pool.get().unwrap();
///             let reply = redis::cmd("PING").query::<String>(conn.deref()).unwrap();
///             assert_eq!("PONG", reply);
///         }));
///     }
///
///     for h in handles {
///         h.join().unwrap();
///     }
/// }
/// ```
// This pull request: https://github.com/mitsuhiko/redis-rs/pull/47
// adds #[derive(Debug)] to redis::ConnectionInfo so it can be used here.
//#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: redis::ConnectionInfo
}

impl RedisConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See `redis::Client::open` for a description of the parameter
    /// types.
    pub fn new<T: redis::IntoConnectionInfo>(params: T)
            -> Result<RedisConnectionManager, redis::RedisError> {
        match params.into_connection_info() {
            Ok(connection_info) => Ok(RedisConnectionManager {
                connection_info: connection_info
            }),
            Err(err) => Err(err)
        }
    }
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = Error;

    fn connect(&self) -> Result<redis::Connection, Error> {
        // This pull request: https://github.com/mitsuhiko/redis-rs/pull/47
        // adds #[derive(Clone)] to redis::ConnectionInfo so it can be used here...
        //redis::Client::open(self.connection_info.clone()).map_err(Error::Other)
        // ...instead of having to do these 7 lines.
        let connection_info = redis::ConnectionInfo {
            addr:    self.connection_info.addr.clone(),
            db:      self.connection_info.db,
            passwd:  self.connection_info.passwd.clone()
        };

        match redis::Client::open(connection_info) {
            Ok(client) => {
                client.get_connection().map_err(Error::Other)
            },
            Err(err) => Err(Error::Other(err))
        }
    }

    fn is_valid(&self, conn: &mut redis::Connection) -> Result<(), Error> {
        match redis::cmd("PING").query(conn.deref()) {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::Other(err))
        }
    }

    fn has_broken(&self, _conn: &mut redis::Connection) -> bool {
        false
    }
}
