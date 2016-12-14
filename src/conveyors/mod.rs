extern crate iron;

pub mod admin;
pub mod frontend;

use self::admin::Admin;
use self::frontend::FrontEnd;

use std::net::TcpListener;

pub struct Conveyors {
    admin_api: Option<Admin>,
    tcp_frontends: Vec<FrontEnd>,
}

impl Conveyors {
    pub fn new() -> Conveyors {
        Conveyors { tcp_frontends: vec!(), admin_api: None }
    }

    pub fn admin_bind(&mut self, bind: String) -> &Conveyors {
        self.admin_api = Some(Admin::new(bind));
        self
    }

    pub fn add_tcp_frontend(&mut self, fe: FrontEnd) -> &Conveyors {
        self.tcp_frontends.push(fe);
        self
    }

    fn start_admin_server(&mut self) {
        info!("admin server pre-start");
        if let Some(ref mut admin) = self.admin_api {
            info!("Starting admin server");
            admin.start();
        }
    }

    pub fn start(&mut self) {
        self.start_admin_server();
    }
}
