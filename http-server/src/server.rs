use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

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
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

                            //let res: &Result<Request, _> = &buffer[..].try_into();

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse request: {}", e)
                            }
                            
                        },
                        Err(e) => println!("Failed to read from Connection: {}", e)
                    }


                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }

    }
}
