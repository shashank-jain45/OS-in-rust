#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");
    loop {}
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop{}
}



//instead of using cargo build, use cargo build --target wasm32-unknown-unknown
//if that gives an error, rustup target add wasm32-unknown-unknown first.
