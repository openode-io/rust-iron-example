extern crate iron;
extern crate params;

use iron::prelude::*;
use iron::status;
use params::{Params, Value};

fn handler(req: &mut Request) -> IronResult<Response> {
    let params_map = req.get_ref::<Params>().unwrap();
    let name = match params_map.find(&["name"]) {
        Some(&Value::String(ref s)) => { s.clone() },
        _ => String::from("world")
    };

    Ok(Response::with(
        (status::Ok, format!("Hello, {}!", name))
    ))
}

fn main() {
    Iron::new(handler).http("0.0.0.0:80").unwrap();
}
