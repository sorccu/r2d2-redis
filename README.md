r2d2-redis
=============

[![travis-ci.org](https://travis-ci.org/nevdelap/r2d2-redis.svg)](https://travis-ci.org/nevdelap/r2d2-redis)[![crates.io](http://meritbadge.herokuapp.com/r2d2-redis)](https://crates.io/crates/r2d2-redis)

[redis-rs](https://github.com/mitsuhiko/redis-rs) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool *totally* based on Steven Fackler's [r2d2-postgres](https://github.com/sfackler/r2d2-postgres). All props to him.

Documentation is available at https://nevdelap.github.io/r2d2-redis/doc/r2d2_redis

# Example

```rust
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

use std::default::Default;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use r2d2_redis::RedisConnectionManager;

fn main() {
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

    let mut handles = vec![];

    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let conn = pool.get().unwrap();
            let reply = redis::cmd("PING").query::<String>(conn.deref()).unwrap();
            assert_eq!("PONG", reply);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```
