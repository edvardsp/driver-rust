
#![allow(dead_code)]

use std::io;

use hw_io::HwIo;

pub struct ElevIo {
    io: HwIo,
}

pub type Floor = usize;
const N_FLOORS: usize = 4;
const TOP: usize = N_FLOORS - 1;
const SEC_TOP: usize = N_FLOORS - 2;

#[derive(Copy, Clone)]
pub enum Button {
    CallUp(Floor),
    CallDown(Floor),
    Internal(Floor),
}

#[derive(Copy, Clone)]
pub enum MotorDir {
    Up,
    Down,
    Stop,
}

#[derive(Copy, Clone)]
pub enum Light {
    On,
    Off,
}

#[derive(Copy, Clone)]
pub enum Signal {
    High,
    Low,
}

impl Signal {
    pub fn new(value: usize) -> Self {
        if value == 0 { Signal::Low }
        else          { Signal::High }
    }
}

const MOTOR: usize = 0x100;
const MOTORDIR: usize = 0x315;
const MOTOR_SPEED: usize = 200;

impl ElevIo {
    pub fn new() -> io::Result<Self> {
        let elev = ElevIo { io: HwIo::new()? };
        elev.set_all_light(Light::Off)?;
        elev.set_floor_light(0)?;
        Ok(elev)
    }

    pub fn set_motor_dir(&self, dir: MotorDir) -> io::Result<()> {
        match dir {
            MotorDir::Up => self.io.write_analog(MOTOR, 0)?,
            MotorDir::Down => {
                self.io.clear_bit(MOTORDIR)?;
                self.io.write_analog(MOTOR, MOTOR_SPEED)?;
            },
            MotorDir::Stop => {
                self.io.set_bit(MOTORDIR)?;
                self.io.write_analog(MOTOR, MOTOR_SPEED)?;
            },
        };
        Ok(())
    }

    pub fn set_all_light(&self, mode: Light) -> io::Result<()> {
        for floor in 0..N_FLOORS {
            if floor != TOP { self.set_button_light(Button::CallUp(floor), mode)?; }
            if floor != 0   { self.set_button_light(Button::CallDown(floor), mode)?; }
            self.set_button_light(Button::Internal(floor), mode)?;
        }
        self.set_stop_light(mode)?;
        self.set_door_light(mode)?;
        Ok(())
    }

    pub fn set_button_light(&self, button: Button, mode: Light) -> io::Result<()> {
        const CALL_UP_ADDR: [usize; 3]   = [ 0x309, 0x308, 0x306 ];
        const CALL_DOWN_ADDR: [usize; 3] = [ 0x307, 0x305, 0x304 ];
        const INTERNAL_ADDR: [usize; 4]  = [ 0x313, 0x312, 0x311, 0x310 ];
        let addr = match button {
            Button::CallUp(floor @ 0...SEC_TOP) => CALL_UP_ADDR[floor],
            Button::CallDown(floor @ 1...TOP) => CALL_DOWN_ADDR[floor-1],
            Button::Internal(floor @ 0...TOP) => INTERNAL_ADDR[floor],
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "given floor is not supported for given button")),
        };
        match mode {
            Light::On => self.io.set_bit(addr)?,
            Light::Off => self.io.clear_bit(addr)?,
        };
        Ok(())
        
    }

    pub fn set_floor_light(&self, floor: Floor) -> io::Result<()> {
        const FLOOR_LIGHT_ADDR: [usize; 2] = [ 0x300, 0x301 ];
        if floor > TOP {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "given floor is not supported"));
        }
        if floor & 0x2 != 0 { self.io.set_bit(FLOOR_LIGHT_ADDR[0])?; } 
        else                { self.io.clear_bit(FLOOR_LIGHT_ADDR[0])?; }
        if floor & 0x1 != 0 { self.io.set_bit(FLOOR_LIGHT_ADDR[1])?; }
        else                { self.io.clear_bit(FLOOR_LIGHT_ADDR[1])?; }
        Ok(())
    }

    pub fn set_door_light(&self, mode: Light) -> io::Result<()> {
        const DOOR_LIGHT_ADDR: usize = 0x303;
        match mode {
            Light::On => self.io.set_bit(DOOR_LIGHT_ADDR)?,
            Light::Off => self.io.clear_bit(DOOR_LIGHT_ADDR)?,
        }
        Ok(())
    }

    pub fn set_stop_light(&self, mode: Light) -> io::Result<()> {
        const STOP_LIGT_ADDR: usize = 0x314;
        match mode {
            Light::On => self.io.set_bit(STOP_LIGT_ADDR)?,
            Light::Off => self.io.clear_bit(STOP_LIGT_ADDR)?,
        }
        Ok(())
    }

    pub fn get_button_signal(&self, button: Button) -> io::Result<Signal> {
        const CALL_UP_ADDR: [usize; 3] = [ 0x317, 0x316, 0x201 ];
        const CALL_DOWN_ADDR: [usize; 3] = [ 0x200, 0x202, 0x203 ];
        const INTERNAL_ADDR: [usize; 4] = [ 0x312, 0x320, 0x319, 0x318 ];
        let addr = match button {
            Button::CallUp(floor @ 0...SEC_TOP) => CALL_UP_ADDR[floor],
            Button::CallDown(floor @ 1...TOP) => CALL_DOWN_ADDR[floor-1],
            Button::Internal(floor @ 0...TOP) => INTERNAL_ADDR[floor],
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "given floor is not supported for given button")),
        };
        let value = self.io.read_bit(addr)?;
        Ok(Signal::new(value))
    }

    pub fn get_floor_signal(&self, floor: Floor) -> io::Result<Signal> {
        const FLOOR_SENSOR_ADDR: [usize; 4] = [ 0x204, 0x205, 0x206, 0x207 ];
        if floor > TOP {
        }
        let value = match floor {
            0...TOP => self.io.read_bit(FLOOR_SENSOR_ADDR[floor])?,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "given floor is not supported")),
        };
        Ok(Signal::new(value))
        
    }

    pub fn get_stop_signal(&self) -> io::Result<Signal> {
        const STOP_SENSOR_ADDR: usize = 0x322;
        Ok(Signal::new(self.io.read_bit(STOP_SENSOR_ADDR)?))
    }

    pub fn get_obstr_signal(&self) -> io::Result<Signal> {
        const OBSTR_SENSOR_ADDR: usize = 0x323;
        Ok(Signal::new(self.io.read_bit(OBSTR_SENSOR_ADDR)?))
    }
    
}

#[cfg(test)]
mod tests {
    use super::ElevIo;

    #[test]
    fn test_elev_io_init() {
        if let Err(err) = ElevIo::new() {
            println!("{}", err);
        }
        
    }
}

