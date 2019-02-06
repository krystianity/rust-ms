use log::info;

use std::io;
use std::thread;

mod cache;
mod httpclient;
mod httpserver;
mod kafka;
mod msbase;

use httpclient as http_c;
use httpserver as http_s;
use kafka::consumer;
use kafka::producer;
use msbase::config;

use redis;
use redis::Commands; //import traits for con.get and con.set

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
        Err(e) => println!("Failed: {}.", e),
    }
}

fn execute() -> Result<(), io::Error> {
    /* ## Logger ## */

    env_logger::init();

    /* ## Loading JSON Configuration ## */

    let conf = config::get_config(None).expect("Failed to load JSON config.");

    /* ## Simple Redis Actions ## */

    let redis_host = conf["redis"]["host"]
        .as_str()
        .expect("redis host missing in config.");
    let redis_port = conf["redis"]["port"]
        .as_i64()
        .expect("redis port missing in config.");
    let redis = match cache::Cache::new(redis_host, redis_port) {
        Ok(result) => result,
        Err(error) => return Err(cache::error_to_io(error)),
    };

    let redis_key = "hans";

    let _ = redis.set(redis_key, "peter"); //set fn is wrapped

    let key_result: Result<String, redis::RedisError> = redis.con.get(redis_key); //get fn is not wrapped
    match key_result {
        Ok(result) => info!("Key val is: {}.", result),
        Err(error) => return Err(cache::error_to_io(error)),
    }

    let _ = redis.del(redis_key); //del fn is wrapped

    /* ## MySQL + ORM ## */

    //TODO

    /* ## Kafka Consumer/Producer ## */

    kafka::log_version_info();

    let kafka_consumer_thread = thread::spawn(move || {
        //TODO implement tokio async version
        let conf = config::get_config(None).expect("Failed to load JSON config.");
        let brokers = conf["kafka"]["brokers"]
            .as_str()
            .expect("kafka brokers missing in config.");
        let group_id = conf["kafka"]["group_id"]
            .as_str()
            .expect("kafka group_id missing in config.");
        let kafka_topic = conf["kafka"]["topic"]
            .as_str()
            .expect("kafka topic missing in config.");
        let kafka_consumer = consumer::MSConsumer::new(brokers, group_id).unwrap();
        let _ = kafka_consumer.consume(&[kafka_topic]);
    });

    let brokers = conf["kafka"]["brokers"]
        .as_str()
        .expect("kafka brokers missing in config.");
    let kafka_topic = conf["kafka"]["topic"]
        .as_str()
        .expect("kafka topic missing in config.");

    let kafka_producer = producer::MSProducer::new(brokers)?;
    let _ = kafka_producer.produce(kafka_topic, "rustkey1", "rustvalue1", 0);
    let _ = kafka_producer.produce(kafka_topic, "rustkey2", "rustvalue2", 0);
    let _ = kafka_producer.produce(kafka_topic, "rustkey3", "rustvalue3", 0);

    /* ## HTTP Server ## */

    let webserver_thread = thread::spawn(move || {
        //TODO move this stuff out of closure, static access?
        let conf = config::get_config(None).expect("Failed to load JSON config.");
        let http_host = conf["http"]["host"]
            .as_str()
            .expect("http host missing in config.");
        let http_port = conf["http"]["port"]
            .as_i64()
            .expect("http port missing in config.");
        http_s::run(http_host, http_port);
    });

    /* ## HTTP Client ## */

    let http_host = conf["http"]["host"]
        .as_str()
        .expect("http host missing in config.");
    let http_port = conf["http"]["port"]
        .as_i64()
        .expect("http port missing in config.");
    let client = http_c::HttpClient::new()?;

    let http_url = &format!("http://{}:{}/", http_host, http_port);
    let mut res = client.get(http_url).unwrap();
    info!("GET: {}, {}", res.status(), res.text().unwrap());

    let mut res = client
        .post(http_url, String::from("{\"hi\":\"bye\"}"))
        .unwrap();
    info!("POST: {}, {}", res.status(), res.text().unwrap());

    /* ## Clean-up ## */

    let _ = webserver_thread.join(); //awaits http server endlessly
    let _ = kafka_consumer_thread.join();
    Ok(())
}
