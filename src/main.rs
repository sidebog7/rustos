
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;


#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    loop {}
}
