pub struct Server {
    addr: String
}

impl Server{
    pub fn new(addr: String) -> Self { // could return Server
        Self { // or Server
            addr  // addr: addr implicitely, for same variable name
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr)
    }
}
