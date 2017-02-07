
extern crate driver;

use std::process;

use driver::elev_io::*;

fn main() {
    println!("Hello, world!");
    let elev_io: ElevIo = match ElevIo::new() {
        Ok(elev) => elev,
        Err(err) => {
            println!("Init of HW failed with Error: {}", err);
            process::exit(1);
        },
    };
    
    elev_io.set_motor_dir(MotorDir::Up).unwrap();

    const SEC_TOP: usize = N_FLOORS - 1;
    loop {
        match elev_io.get_floor_signal().unwrap() {
            Floor::At(SEC_TOP) => elev_io.set_motor_dir(MotorDir::Up).unwrap(),
            Floor::At(0) => elev_io.set_motor_dir(MotorDir::Down).unwrap(),
            _ => {}
        }

        match elev_io.get_stop_signal().unwrap() {
            Signal::High => { 
                elev_io.set_motor_dir(MotorDir::Stop).unwrap();
                process::exit(0);
            },
            _ => {}
        }
    }
}
