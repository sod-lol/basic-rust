

use std::io::ErrorKind;
use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    
    
    fn get_username() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}
