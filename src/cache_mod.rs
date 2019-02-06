pub mod cache {
    use std::io::{Error, ErrorKind};

    use log::info;
    use redis::Commands;

    pub struct Cache {
        //client: redis::Client,
        pub con: redis::Connection
    }

    impl Cache {

        pub fn new(host: &str, port: i64) -> Result<Cache, redis::RedisError> {
            let constr: &str = &format!("redis://{}:{}/", host, port);
            let client = redis::Client::open(constr)?;
            let con = client.get_connection()?;
            info!("Redis client connected to {}.", constr);
            Ok(Cache {/*client,*/ con})
        }

        pub fn set(&self, key: &str, val: &str) -> Result<(), redis::RedisError> { 
            let _ : () = self.con.set(key, val)?;
            Ok(())
        }

        pub fn del(&self, key: &str) -> Result<(), redis::RedisError> { 
            let _ : () = self.con.del(key)?;
            Ok(())
        }
    }

    pub fn error_to_io(error: redis::RedisError) -> Error {
        Error::new(ErrorKind::Other, error.category())
    }
}