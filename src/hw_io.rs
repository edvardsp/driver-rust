
#![allow(dead_code)]

use std::io;
use std::ffi::CString;

use libc::{c_int, c_char, c_uint};

// comedilib opaque types
enum ComediT {}

#[link(name = "comedi")]
extern {
    fn comedi_open(filename: *const c_char) -> *const ComediT;
    fn comedi_dio_config(it: *const ComediT, subd: c_uint, chan: c_uint, dir: c_uint) -> c_int;
    fn comedi_dio_write(it: *const ComediT, subd: c_uint, chan: c_uint, bit: c_uint) -> c_int;
    fn comedi_dio_read(it: *const ComediT, subd: c_uint, chan: c_uint, bit: *mut c_uint) -> c_int;
    fn comedi_data_write(it: *const ComediT, subd: c_uint, chan: c_uint, range: c_uint, aref: c_uint, data: c_uint) -> c_int;
    fn comedi_data_read(it: *const ComediT, subd: c_uint, chan: c_uint, range: c_uint, aref: c_uint, data: *mut c_uint) -> c_int;
}

pub struct HwIo {
    it: *const ComediT,
}

const AREF_GROUND: c_uint = 0;
const INPUT: c_uint = 0;
const OUTPUT: c_uint = 1;

struct Port {
    subdev:      c_uint,
    chan_offset: c_uint,
    dir:         c_uint,
}

const PORTS: [Port; 4] = [
    Port { subdev: 2, chan_offset:  0, dir: INPUT },
    Port { subdev: 3, chan_offset:  0, dir: OUTPUT },
    Port { subdev: 3, chan_offset:  8, dir: OUTPUT },
    Port { subdev: 3, chan_offset: 16, dir: INPUT },
];

impl HwIo {
    pub fn new() -> io::Result<Self> {
        let dev = CString::new("/dev/comedi0").unwrap();
        let it = unsafe { comedi_open(dev.as_ptr()) };
        if it.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "comedi_open failed"));
        }

        for offset in 1..8 {
            for port in &PORTS {
                let status = unsafe { 
                    comedi_dio_config(it, port.subdev, offset + port.chan_offset, port.dir)
                };
                if status != 0 {
                    return Err(io::Error::new(
                            io::ErrorKind::Other, 
                            format!("comedi_dio_config failed, ({},{},{})", 
                                    port.subdev, 
                                    offset + port.chan_offset, 
                                    port.dir)));
                }
            }
        }

        Ok(HwIo { it: it })
    } 

    pub fn set_bit(&self, channel: usize) -> io::Result<()> {
        let ch = channel as c_uint;
        let ret = unsafe { comedi_dio_write(self.it, ch >> 8, ch & 0xff, 1) };
        if ret == 1 { Ok(()) }
        else { Err(io::Error::new(io::ErrorKind::Other, format!("set_bit: comedi_dio_write[0x{:x}]) failed", channel))) }
    }

    pub fn clear_bit(&self, channel: usize) -> io::Result<()> {
        let ch = channel as c_uint;
        let ret = unsafe { comedi_dio_write(self.it, ch >> 8, ch & 0xff, 0) };
        if ret == 1 { Ok(()) }
        else { Err(io::Error::new(io::ErrorKind::Other, format!("clear_bit: comedi_dio_write failed[0x{:x}]", channel))) }
    }

    pub fn read_bit(&self, channel: usize) -> io::Result<usize> {
        let ch = channel as c_uint;
        let mut data: c_uint = 0;
        let ret = unsafe { comedi_dio_read(self.it, ch >> 8, ch & 0xff, &mut data as *mut c_uint) };
        if ret == 1 { Ok(data as usize) }
        else { Err(io::Error::new(io::ErrorKind::Other, format!("read_bit: comedi_dio_read[0x{:x}] failed", channel))) }
    }

    pub fn write_analog(&self, channel: usize, value: usize) -> io::Result<()> {
        let ch = channel as c_uint;
        let ret = unsafe { comedi_data_write(self.it, ch >> 8, ch & 0xff, 0, AREF_GROUND, value as c_uint) };
        if ret == 1 { Ok(()) }
        else { Err(io::Error::new(io::ErrorKind::Other, format!("wrte_analog: comedi_data_write[0x{:x}] failed", channel))) }
    }

    pub fn read_analog(&self, channel: usize) -> io::Result<usize> {
        let ch = channel as c_uint;
        let mut data: c_uint = 0;
        let ret = unsafe { comedi_data_read(self.it, ch >> 8, ch & 0xff, 0, AREF_GROUND, &mut data as *mut c_uint) };
        if ret == 1 { Ok(data as usize) }
        else { Err(io::Error::new(io::ErrorKind::Other, format!("read_analog: comedi_data_read[0x{:x}] failed", channel))) }
    }
}

#[cfg(test)]
mod tests {
    use super::HwIo;

    #[test]
    fn test_hw_io_init() {
        assert!(HwIo::new().is_ok(), "ElevIo::new failed");
    }
}
