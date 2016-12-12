extern crate iron;

use iron::prelude::*;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

pub fn start() {
    let mut chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:9001").unwrap();
}
