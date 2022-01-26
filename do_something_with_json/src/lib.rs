use std::error::Error;
use std::fmt;

pub fn run() -> Result<(), Box<Error>> {

    let x: String = "abc";
    
	let e = MyError { msg: String::from("a random error message") };
	Err(Box::new(e))
}


//////////////////////////////////////////////////
// MyError
//////////////////////////////////////////////////

struct MyError {
	msg: String,
}

impl Error for MyError {
	 fn description(&self) -> &str {
	 	&self.msg
	 }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError {{ msg: {} }}", self.msg)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

//////////////////////////////////////////////////
// Tests
//////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
    }

    #[test]
    fn test_bar() {
    	panic!("panic by design");
    }
}
