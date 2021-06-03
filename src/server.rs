use std::net::TcpListener;
use std::io::Read;

pub struct Server {

    addr: String,
}

// implementation holds struct functionality
impl Server {
    
    pub fn new(addr: String) -> Self {
        // could also be named 'Server'
        Self {
            addr: addr
        }
    }

    // methods accept self as first parameter
    // would be 'this' in javascript
    // run fn takes ownership of the struct because it accepts self
    // if we don't want to deallocate after, we can take a reference of '&self'
    pub fn run(self) {
        println!("Listening on {}", self.addr); 

        // creating new listening using bind function
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request: {}", String::from_utf8_lossy(&buffer))
                        },
                        Err(e) =>  println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

}