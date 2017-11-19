#[macro_use] 
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate env_logger;

use std::io;
use std::thread;

mod base_mod;
mod httpserver_mod;
mod cache_mod;
mod httpclient_mod;

use base_mod::msbase::config as config;
use httpserver_mod::mshttp_s as http_s;
use httpclient_mod::mshttp_c as http_c;
use cache_mod::cache as cache;

use cache_mod::cache::redis;
use cache_mod::cache::redis::Commands; //import traits for con.get and con.set

/*
    rust for Node.js developers:
    - Result<val,err> -> Ok(), Err() => Javascript Callback
    - Option<val> -> Some(), None, unwrap, unwrap_or => Java Optional
    - dont end with ; for returns, as this will cause the fn to return ()
    - macros need to be placed in the main rs file once
    - a module is imported via "mod name"
    - the order of use:: and mod doesnt care
    - if a module uses crates they have to be accessed via use self::crate
    - try! unwraps a Result<> but returns early, so that the fn has to return a Result<> as well
    - traits === interfaces
    - struct + impl === class
    - second let definition overwrites first without error or warning
*/

fn main() {
    match execute() {
        Ok(_) => println!("Successfull."),
        Err(e) => println!("Failed: {}.", e)
    }
}

fn execute() -> Result<(), io::Error> {

    /* ## Logger ## */

    env_logger::init().expect("Failed to setup logger.");

    /* ## Loading JSON Configuration ## */

    let conf = config::get_config(None).expect("Failed to load JSON config.");

    /* ## Simple Redis Actions ## */

    let redis_host = conf["redis"]["host"].as_str().expect("redis host missing in config.");
    let redis_port = conf["redis"]["port"].as_i64().expect("redis port missing in config.");
    let redis = match cache::Cache::new(redis_host, redis_port) {
        Ok(result) => result,
        Err(error) => return Err(cache::error_to_io(error))
    };

    let redis_key = "hans";

    let _ = redis.set(redis_key, "peter"); //set fn is wrapped

    let key_result: Result<String, redis::RedisError> = redis.con.get(redis_key);
    match key_result {
        Ok(result) => info!("Key val is: {}.", result),
        Err(error) => return Err(cache::error_to_io(error))
    }

    let _ = redis.del(redis_key);

    /* ## MySQL + ORM ## */

    //TODO

    /* ## Kafka Consumer/Producer ## */

    //TODO

    /* ## HTTP Server ## */
    
    let webserver_thread = thread::spawn(move || {
        //TODO move this stuff out of closure, static access?
        let conf = config::get_config(None).expect("Failed to load JSON config.");
        let http_host = conf["http"]["host"].as_str().expect("http host missing in config.");
        let http_port = conf["http"]["port"].as_i64().expect("http port missing in config.");
        http_s::run(http_host, http_port);
    });

    /* ## HTTP Client ## */

    let http_host = conf["http"]["host"].as_str().expect("http host missing in config.");
    let http_port = conf["http"]["port"].as_i64().expect("http port missing in config.");
    let client = http_c::HttpClient::new()?;

    let http_url = &format!("http://{}:{}/", http_host, http_port);
    let mut res = client.get(http_url).unwrap();
    info!("GET: {}, {}", res.status(), res.text().unwrap());

    let mut res = client.post(http_url, String::from("{\"hi\":\"bye\"}")).unwrap();
    info!("POST: {}, {}", res.status(), res.text().unwrap());

    /* ## Clean-up ## */

    let _ = webserver_thread.join(); //awaits http server endlessly
    Ok(())
}