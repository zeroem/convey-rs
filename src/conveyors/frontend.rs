use std::net::TcpListener;

pub struct FrontEnd {
    listener: TcpListener,
}

impl FrontEnd {
    pub fn new(addr: &str) -> FrontEnd {
        if let Ok(listener) = TcpListener::bind(addr) {
            FrontEnd {
                listener: listener,
            }
        } else {
            panic!("error binding socket")
        }
    }
}
