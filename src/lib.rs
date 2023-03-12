//! Redis support for the `r2d2` connection pool.
pub use r2d2;
pub use redis;

use redis::{ConnectionLike, RedisError};

/// An `r2d2::ConnectionManager` for `redis::Client`s.
///
/// ## Example
///
/// ```
/// use std::ops::DerefMut;
/// use std::thread;
/// use r2d2_redis::{r2d2, redis, RedisConnectionManager};
///
/// let (host, port) = (
///     std::env::var("REDIS_HOST").unwrap(),
///     std::env::var("REDIS_PORT").unwrap(),
/// );
/// let uri = format!("redis://{host}:{port}");
/// let manager = RedisConnectionManager::new(uri).unwrap();
/// let pool = r2d2::Pool::builder()
///     .build(manager)
///     .unwrap();
/// let mut handles = vec![];
/// for _i in 0..10i32 {
///     let pool = pool.clone();
///     handles.push(thread::spawn(move || {
///         let mut conn = pool.get().unwrap();
///         let reply = redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
///         // Alternatively, without deref():
///         // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
///         assert_eq!("PONG", reply);
///     }));
/// }
/// for h in handles {
///     h.join().unwrap();
/// }
//// ```
#[derive(Debug)]
pub struct RedisConnectionManager {
    connection_info: redis::ConnectionInfo,
}

impl RedisConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See [redis::Client] for a description of the parameter types.
    pub fn new<T: redis::IntoConnectionInfo>(
        params: T,
    ) -> Result<RedisConnectionManager, redis::RedisError> {
        let connection_info = params.into_connection_info()?;
        Ok(RedisConnectionManager { connection_info })
    }
}

impl r2d2::ManageConnection for RedisConnectionManager {
    type Connection = redis::Connection;
    type Error = RedisError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = redis::Client::open(self.connection_info.clone())?;
        client.get_connection()
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        redis::cmd("PING").query(conn)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        !conn.is_open()
    }
}
