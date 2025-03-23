#![no_std] 
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, world!";

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    HELLO.iter().enumerate().for_each(|(i, &ch)| {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = ch;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    });

    loop {}
}
