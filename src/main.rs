
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate driver;

use driver::elev_io::*;

fn main() {
    println!("Hello, world!");
    let elev_io = ElevIo::new()
        .expect("Init of HW failed");
    
    elev_io.set_motor_dir(MotorDir::Up)
           .expect("Set MotorDir failed");

    const SEC_TOP: usize = N_FLOORS - 1;
    loop {
        match elev_io.get_floor_signal()
                     .expect("Get FloorSignal failed") {
            Floor::At(SEC_TOP) => elev_io.set_motor_dir(MotorDir::Down)
                                         .expect("Set MotorDir failed"),
            Floor::At(0) => elev_io.set_motor_dir(MotorDir::Up)
                                   .expect("Set MotorDir Failed"),
            _ => {}
        }

        if let Signal::High = elev_io.get_stop_signal()
                                     .expect("Get StopSignal failed") {
            elev_io.set_motor_dir(MotorDir::Stop)
                   .expect("Set MotorDir failed");
            return;
        }
    }
}
