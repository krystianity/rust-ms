#[macro_use]
extern crate serde_json;

use std::io;

mod base_mod;
mod http_mod;
mod cache_mod;

use base_mod::msbase::config as config;
use http_mod::mshttp as http;
use cache_mod::cache as cache;

use cache_mod::cache::redis::Commands; //import traits for con.get and con.set

/*
    rust for Node.js developers:
    - Result<val,err> -> Ok(), Err() => Javascript Callback
    - Option<val> -> Some(), None, unwrap, unwrap_or => Java Optional
    - dont end with ; for returns, that kills Options
    - a module is imported via "mod name"
    - the order of use:: and mod doesnt care
    - if a module uses crates they have to be accessed via use self::crate
    - try! unwraps a Result<> but returns early, so that the fn has to return a Result<> as well
    - traits === interfaces
    - struct + impl === class
*/

fn main() {
    match execute() {
        Ok(_) => println!("Successfull."),
        Err(e) => println!("Failed: {}.", e)
    }
}

fn execute() -> Result<(), io::Error> {

    /* ## Loading JSON Configuration ## */

    let conf = config::get_config(None).expect("Failed to load JSON config.");

    /* ## Simple Redis Actions ## */

    let redis_host = conf["redis"]["host"].as_str().expect("redis host missing in config.");
    let redis_port = conf["redis"]["port"].as_i64().expect("redis port missing in config.");

    let redis = match cache::Cache::new(redis_host, redis_port) {
        Ok(result) => result,
        Err(error) => return Err(cache::error_to_io(error))
    };

    let _ = redis.set("hans", "peter"); //set fn is wrapped

    let val: String = match redis.con.get("hans") {
        Ok(result) => result,
        Err(error) => return Err(cache::error_to_io(error))
    };

    println!("Key val is: {}.", val);

    /* ## HTTP Server ## */
    
    let http_host = conf["http"]["host"].as_str().expect("http host missing in config.");
    let http_port = conf["http"]["port"].as_i64().expect("http port missing in config.");
    http::run(http_host, http_port);

    Ok(())
}