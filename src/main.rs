#![no_std]
#![no_main]
#![cfg(not(test))]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

//instead of using cargo build, use cargo build --target wasm32-unknown-unknown
//if that gives an error, rustup target add wasm32-unknown-unknown first.
