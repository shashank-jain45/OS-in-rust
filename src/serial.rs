//This module is for the serial port implementation which has to be used instead of the TCP model to run.



use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref SERIAL1: Mutex<SerialPort>={
        let mut serial_port=unsafe{SerialPort::new(0x3F8)};
        serial_port.init();
        Mutex::new(serial_port)
    };
}
//0x3F8 is the standard port for first serail interface.
//uart uses multiple i/o ports.
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments){
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(||{
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! serial_print{
    ($($arg:tt)*) =>{
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println{
    ()=>($crate::serial_print!("\n"));
    ($fmt:expr)=>($crate::serial_print!(concat!($fmt,"\n")));
    ($fmt:expr,$($arg:tt)*)=>($crate::serial_print!(
        concat!($fmt,"\n"),$($arg)*));
}