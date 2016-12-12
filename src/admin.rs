extern crate iron;

use iron::prelude::*;

mod admin;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn start() -> {
    return Iron::new().http("localhost:9001").unwrap()
}
