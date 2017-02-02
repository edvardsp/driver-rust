
extern crate driver;

use driver::elev_io::ElevIo;

fn main() {
    println!("Hello, world!");
    if let Err(err) = ElevIo::new() {
        println!("{}", err);
    }
}
