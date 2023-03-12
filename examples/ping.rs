use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use std::ops::DerefMut;
use std::thread;

fn main() {
    let (host, port) = (
        std::env::var("REDIS_HOST").unwrap(),
        std::env::var("REDIS_PORT").unwrap(),
    );
    let uri = format!("redis://{host}:{port}");
    let manager = RedisConnectionManager::new(uri).unwrap();
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
