r2d2-redis
=============

[![travis-ci.org](https://travis-ci.org/sorccu/r2d2-redis.svg)](https://travis-ci.org/sorccu/r2d2-redis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![crates.io](http://meritbadge.herokuapp.com/r2d2-redis)](https://crates.io/crates/r2d2-redis) [![Documentation](https://docs.rs/r2d2_redis/badge.svg?version=0.12.0)](https://docs.rs/r2d2_redis/0.12.0)

[redis-rs](https://github.com/mitsuhiko/redis-rs) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool *totally* based on Steven Fackler's [r2d2-postgres](https://github.com/sfackler/r2d2-postgres). All props to him.

Documentation is available [here](https://docs.rs/r2d2_redis/0.12.0).

[r2d2-redis](https://github.com/sorccu/r2d2-redis) was originally developed by [@nevdelap](https://github.com/nevdelap), who has since transferred the repo here due to no longer having enough time to maintain it. Thanks for all your hard work, [@nevdelap](https://github.com/nevdelap)!

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
r2d2_redis = "0.12.0"
```

Then check out the examples below.

# Examples

See [examples](examples) for runnable examples.

## Standard usage

This example shows a standard use case with convenience methods provided by `redis::Commands`. You'll note that it's practically the same as if you were using the redis crate directly. Thanks to the `Deref` trait, you'll be able to call any `Connection` method directly on a pooled connection.

Run with `cargo run --example counter`:

```rust
extern crate r2d2_redis;

use std::thread;

use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

fn main() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    let mut handles = vec![];

    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            let n: i64 = conn.incr("counter", 1).unwrap();
            println!("Counter increased to {}", n);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```

## Manual query building

Unfortunately there are cases when the `Deref` trait cannot be used. This usually happens when you need to pass the redis connection somewhere else, such as when building queries manually and/or if the redis crate doesn't expose a convenience method for a particular command (e.g. `PING`). In these cases you must use and call the `Deref` trait directly.

Run with `cargo run --example ping`:

```rust
extern crate r2d2_redis;

use std::ops::DerefMut;
use std::thread;

use r2d2_redis::{r2d2, redis, RedisConnectionManager};

fn main() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    let mut handles = vec![];

    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            let reply = redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
            // Alternatively, without deref():
            // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
            assert_eq!("PONG", reply);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
```
