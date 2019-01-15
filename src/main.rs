
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

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers {} and {}", 42, 1.0/3.0).unwrap();

    loop {}
}
