#![no_std]
#![no_main]
#![cfg(not(test))]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop{}
}
// static HELLO: &[u8] = b"Hello World!";
mod vga_buffer;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
    //    }
    // }
    // vga_buffer::print_something();
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
    // write!(vga_buffer::WRITER.lock(),", same numbers in same order:{} {}",42,1.0/3.0).unwrap();
    println!(" Hello World{}","!");
    loop {}
}

//instead of using cargo build, use cargo build --target wasm32-unknown-unknown
//if that gives an error, rustup target add wasm32-unknown-unknown first.
