#![no_main]
#![no_std]
// Stolen from https://github.com/phip1611/multiboot2-binary-rust/blob/main/src/rust/kernel-bin/src/main.rs
// The way i was attempting to do it was refusing to work, and from what i gather was because rust wasn't including the asm files in the build process.
// This is a much simpler way of doing it, and hopefully it works.

core::arch::global_asm!(include_str!("mb2_header.S"));
core::arch::global_asm!(include_str!("start.S"));

// We can't do anything with panic info yet because we don't have any code to print to the screen.
// In the future, this will print to the screen and to the serial port.
#[panic_handler]
pub fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const VGA_HEAD: *mut u16 = 0xb8000 as *mut u16;

fn set_char_at(x: usize, y: usize, c: u8) {
    unsafe {
        VGA_HEAD
            .add(y * 80 + x)
            .write_volatile((0x0f00 | c as u16).swap_bytes());
    }
}

fn set_chars_at(x: usize, y: usize, s: &[u8]) {
    for (i, &c) in s.iter().enumerate() {
        set_char_at(x + i, y, c);
    }
}

#[no_mangle]
extern "C" fn kstart(mb2_magic: u32, mb2_info_ptr: u32) -> ! {
    for i in 0..80 {
        for j in 0..25 {
            set_char_at(i, j, b"Hello, world!"[i]);
        }
    }
    loop {}
}
