#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

mod vga_buffer;

use core::panic::PanicInfo;
use bootloader::{bootinfo::BootInfo, entry_point};




#[cfg(not(test))] // only compile when the test flag is not set
#[no_mangle]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rustos::interrupts::PICS;

    println!("Hello World{}", "!");

    rustos::gdt::init();
    rustos::interrupts::init_idt();

    unsafe { PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    use rustos::memory;
    use rustos::memory::{create_example_mapping, EmptyFrameAllocator};

    let mut recursive_page_table = unsafe {
        memory::init(boot_info.p4_table_addr as usize)
    };

    create_example_mapping(&mut recursive_page_table, &mut EmptyFrameAllocator);
    unsafe { (0x1900 as *mut u64).write_volatile(0xf021f077f065f04e) };

    println!("It did not crash!");

    rustos::hlt_loop();
}
