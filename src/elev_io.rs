
#![allow(dead_code)]

use std::io;
use std::ffi::CString;

use libc::{c_int, c_char, c_uint};

// coemdilib opaque types
pub enum ComediT {}

#[link(name = "comedi")]
extern {
    fn comedi_open(filename: *const c_char) -> *const ComediT;
    fn comedi_dio_config(it: *const ComediT, subd: c_uint, chan: c_uint, dir: c_uint) -> c_int;
    fn comedi_dio_write(it: *const ComediT, subd: c_uint, chan: c_uint, bit: c_uint) -> c_int;
    fn comedi_dio_read(it: *const ComediT, subd: c_uint, chan: c_uint, bit: *mut c_uint) -> c_int;
    fn comedi_data_write(it: *const ComediT, subd: c_uint, chan: c_uint, range: c_uint, aref: c_uint, data: c_uint) -> c_int;
    fn comedi_data_read(it: *const ComediT, subd: c_uint, chan: c_uint, range: c_uint, aref: c_uint, data: *mut c_uint) -> c_int;
}

struct ElevIo {
    it: *const ComediT,
}

struct Port {
    subdev: usize,
    chan_offset: usize,
    dir: usize,
}

const PORT_1_SUBDEV: c_uint      = 2;
const PORT_1_CHAN_OFFSET: c_uint = 0;
const PORT_1_DIR: c_uint         = 0; // COMEDI_INPUT

const PORT_2_SUBDEV: c_uint      = 3;
const PORT_2_CHAN_OFFSET: c_uint = 0;
const PORT_2_DIR: c_uint         = 1; // COMEDI_OUTPUT

const PORT_3_SUBDEV: c_uint      = 3;
const PORT_3_CHAN_OFFSET: c_uint = 8;
const PORT_3_DIR: c_uint         = 1; // COMEDI_OUTPUT

const PORT_4_SUBDEV: c_uint      = 3;
const PORT_4_CHAN_OFFSET: c_uint = 16;
const PORT_4_DIR: c_uint         = 0; // COMEDI_INPUT

impl ElevIo {
    fn new() -> io::Result<Self> {
        let dev = CString::new("/dev/comedi0").unwrap();
        let it = unsafe { comedi_open(dev.as_ptr()) };
        if it.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "comedi_open failed"));
        }

        let subdev = vec![PORT_1_SUBDEV, PORT_2_SUBDEV, PORT_3_SUBDEV, PORT_4_SUBDEV];
        let chan_offset = vec![PORT_1_CHAN_OFFSET, PORT_2_CHAN_OFFSET, PORT_3_CHAN_OFFSET, PORT_4_CHAN_OFFSET];
        let dir = vec![PORT_1_DIR, PORT_2_DIR, PORT_3_DIR, PORT_4_DIR];

        for offset in 1..8 {
            for port in 0..4 {
                let status = unsafe { comedi_dio_config(it, subdev[port], offset + chan_offset[port], dir[port]) };
                if status != 0 {
                    return Err(io::Error::new(io::ErrorKind::Other, "comedi_dio_config failed"));
                }
            }
        }

        Ok(ElevIo {
            it: it,
        })
    } 

    fn set_bit(&self, channel: usize) {
        unsafe { comedi_dio_write(self.it, channel as c_uint >> 8, channel as c_uint & 0xff, 1) };
    }

    fn clear_bit(&self, channel: usize) {
        unsafe { comedi_dio_write(self.it, channel as c_uint >> 8, channel as c_uint & 0xff, 0) };
    }

    fn write_analog(&self, channel: usize, value: usize) {
        unsafe { comedi_data_write(self.it, channel as c_uint >> 8, channel as c_uint & 0xff, 0, 0, value as c_uint) };
    }

    fn read_bit(&self, channel: usize) -> usize {
        let mut data: c_uint = 0;
        unsafe { comedi_dio_read(self.it, channel as c_uint >> 8, channel as c_uint & 0xff, &mut data as *mut c_uint) };
        data as usize
    }

    fn read_analog(&self, channel: usize) -> usize {
        let mut data: c_uint = 0;
        unsafe { comedi_data_read(self.it, channel as c_uint >> 8, channel as c_uint & 0xff, 0, 0, &mut data as *mut c_uint) };
        data as usize
    }
}

#[cfg(test)]
mod tests {
    use super::ElevIo;

    #[test]
    fn test_elev_io_init() {
        assert!(ElevIo::new().is_ok(), "ElevIo::new failed");
    }
}
