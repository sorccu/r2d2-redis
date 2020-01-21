extern crate r2d2_redis;

use std::ops::DerefMut;
use std::thread;

use r2d2_redis::{r2d2, redis, RedisConnectionManager};

fn main() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let mut handles = vec![];

    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            let reply = redis::cmd("PING")
                .query::<String>(conn.deref_mut())
                .unwrap();
            // Alternatively, without deref():
            // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
            assert_eq!("PONG", reply);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
