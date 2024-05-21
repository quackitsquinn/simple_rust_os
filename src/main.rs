#![no_main]
#![no_std]
// Stolen from https://github.com/phip1611/multiboot2-binary-rust/blob/main/src/rust/kernel-bin/src/main.rs
// The way i was attempting to do it was refusing to work, and from what i gather was because rust wasn't including the asm files in the build process.
// This is a much simpler way of doing it, and hopefully it works.

// ok its several hours later and WHAT THE F- DOES R_X86_64_64 MEAN????
// ITS TELLING ME TO ENABLE -fPIC BUT THATS NOT A THING IN RUSTC OR RUST-LLD
// i am very, very stumped. This was already out of my depth and ive tried the things i can think of.
// the last thing im going to try is to attempt to build multiboot2-binary-rust on my linux machine, and see if that works.
// if it does, it might be a problem with cross compilation on macos. if it doesnt, then i have no idea what to do.
// granted, the version of rust that uses is a nightly version from 2022, but thats recent enough that it shouldnt be a problem.
// some steps so i dont completely forget what i did:
// modify the offset in the linker script from 8M to 1M
// set RUSTFLAGS to include fPIC (didn't work)
// add fPIC to linker flags (didn't work)
// need to do:
// try building on linux
// ok so multiboot2-binary-rust builds on linux, but this doesnt. Somehow i messed up something.
// im so confused
// I have *almost* the exact same code as the other project, but it fails to link. I have no idea why.
// I'm fairly certain that the other project will work on my mac, because i have already tried to build it and ran into an issue with a dependency being wrong.
// fixing the dependency made it build, but it has nothing to do with a linkerscript or anything this bs is.
// last resort:
// make a post on osdev or stackoverflow
// doesn't help that the error message is so vague, and that i cant seem to find any *relevant* information on the internet.
// next step is stackoverflow or the osdev discord, i guess. hopefully one of them have some clue what to do.
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
