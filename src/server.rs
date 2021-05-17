// struct holds data
pub struct Server {
    // data we want struct to hold
    // single string that hold IP and Port

    addr: String,
}

// implementation holds struct functionality
impl Server {
    // either a method or associated function

    // methods in rust always take a special first parameter called self
    // self represents the instance of the struct the method is being called on

    // the other type is associated functions
    // fn's that are associated with the struct type, but they dont need an instance of the struct
    // similar to static methods in other languages
    // double colon to access associated functions

    // we can name this anything, we are choosing 'new'
    // accepts String, Returns Server
    // uppercase Self is an alias for the name of the struct
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
    }

}