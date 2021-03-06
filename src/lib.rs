#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod serial;
#[cfg(not(windows))] // no bang ("!") after the hash ("#")
pub mod interrupts;
pub mod gdt;
pub mod memory;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
