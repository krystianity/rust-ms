#[macro_use]
extern crate serde_json;

mod msbase;
mod mshttp;

use msbase::msbase::config as config;

/*
    rust for Node.js developers:
    Result<val,err> -> Ok(), Err() => Javascript Callback
    Option<val> -> Some(), None, unwrap, unwrap_or => Java Optional
    dont end with ; for returns, that kills Options
    a module is imported via "mod name"
    if a module uses crates they have to be accessed via use self::crate
*/

fn main() {
    let conf = config::get_config(None).unwrap();
    let constr: &str = &format!("localhost:{}", conf.get("port").unwrap());
    mshttp::mshttp::run(constr);
}