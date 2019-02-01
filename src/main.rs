#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

mod vga_buffer;

use core::panic::PanicInfo;


#[cfg(not(test))] // only compile when the test flag is not set
#[no_mangle]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use rustos::interrupts::PICS;

    println!("Hello World{}", "!");

    rustos::gdt::init();
    rustos::interrupts::init_idt();

    unsafe { PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    use rustos::memory::translate_addr;

    const LEVEL_4_TABLE_ADDR: usize = 0o_177777_777_777_777_777_0000;

    println!("0xb8000 -> {:?}", translate_addr(0xb8000, LEVEL_4_TABLE_ADDR));

    println!("0x20010a -> {:?}", translate_addr(0x20010a, LEVEL_4_TABLE_ADDR));

    println!("0x57ac001ffe48 -> {:?}", translate_addr(0x57ac001ffe48,
        LEVEL_4_TABLE_ADDR));



    println!("It did not crash!");

    rustos::hlt_loop();
}
