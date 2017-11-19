pub mod mshttp_s {

    extern crate iron;
    extern crate time;
    extern crate router;
    extern crate bodyparser;
    extern crate persistent;
    extern crate serde_json;

    use self::iron::prelude::*;
    use self::iron::{BeforeMiddleware, AfterMiddleware, typemap};
    use self::iron::status;
    use self::iron::headers::ContentType;
    use self::time::precise_time_ns;
    use self::router::Router;
    use self::persistent::Read;

    const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;
    struct ResponseTime;
    struct ChainJSONParser;

    impl typemap::Key for ResponseTime { type Value = u64; }
    impl typemap::Key for ChainJSONParser { type Value = serde_json::value::Value; }

    impl BeforeMiddleware for ResponseTime {
        fn before(&self, req: &mut Request) -> IronResult<()> {
            req.extensions.insert::<ResponseTime>(precise_time_ns());
            Ok(())
        }
    }

    impl AfterMiddleware for ResponseTime {
        fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
            let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
            println!("Request took: {} ms", (delta as f64) / 1000000.0);
            Ok(res)
        }
    }

    impl BeforeMiddleware for ChainJSONParser {
        fn before(&self, req: &mut Request) -> IronResult<()> {
            let body = req.get::<bodyparser::Json>(); // returns Result<Option>
            match body {
                Ok(Some(json_body)) => {req.extensions.insert::<ChainJSONParser>(json_body); ()},
                Ok(None) => println!("empty body"),
                Err(err) => println!("error parsing body: {:?}", err)
            }
            Ok(())
        }
    }

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "Hello World")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    fn body_handler(req: &mut Request) -> IronResult<Response> {
        let json_body = req.extensions.get::<ChainJSONParser>();
        match json_body {
            Some(ref value) => println!("Parsed body:\n{:?}", *value),
            None => ()
        }
        Ok(Response::with((ContentType::json().0, status::Ok, json_result().to_string())))
    }

    fn json_result() ->  serde_json::value::Value {

        let json_value = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
        });

        json_value
    }

    pub fn run(host: &str, port: i64){

        let mut router = Router::new();

        let mut hello_chain = Chain::new(hello_world);
        hello_chain.link_before(ResponseTime);
        hello_chain.link_after(ResponseTime);

        router.get("/", Chain::new(hello_world), "root");
        router.get("/hello", hello_chain, "hello");

        let mut query_chain = Chain::new(query_handler);
        query_chain.link_before(ResponseTime);
        query_chain.link_after(ResponseTime);

        router.get("/:query", query_chain, "query");

        let mut body_chain = Chain::new(body_handler);
        body_chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
        body_chain.link_before(ChainJSONParser);

        router.post("/", body_chain, "body");
        
        let constr: &str = &format!("{}:{}", host, port);
        info!("Running webserver @ http://{}.", constr);
        Iron::new(router).http(constr).unwrap();
    }
}