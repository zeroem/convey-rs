extern crate iron;

use iron::prelude::*;
use iron::error::HttpResult;
use iron::Listening;
use conveyors::Conveyors;

pub struct Admin {
    listener: Option<Listening>,
    bind: String,
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}


impl Admin {
    pub fn new(bind: String) -> Admin {
        Admin { listener: None, bind: bind }
    }

    pub fn start(&mut self) {
        info!("Starting up admin server");
        let mut chain = Chain::new(hello_world);
        self.listener = Some(Iron::new(chain).http(self.bind.as_str()).unwrap());
    }
}
