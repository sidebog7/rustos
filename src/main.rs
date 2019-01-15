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
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    rustos::gdt::init();
    rustos::interrupts::init_idt();

    //x86_64::instructions::int3();
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    println!("It did not crash!");

    loop {}
}
