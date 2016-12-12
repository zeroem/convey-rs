extern crate iron;

use iron::prelude::*;
use iron::error::HttpResult;
use iron::Listening;


fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

pub fn start() -> Listening {
    info!("Starting up admin server");
    let mut chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:9001").unwrap()
}
