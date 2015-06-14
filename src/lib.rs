#![feature(no_std)]
#![no_std]
#![feature(lang_items, start, libc, core, asm, collections)]

extern crate libc;
extern crate core;
extern crate collections;

use arch::arm::mach_bcm2835::gpio::*;

pub mod arch {
	pub mod arm {
		pub mod mach_bcm2835 {
			pub mod gpio;
}}}

#[no_mangle] // ensure that this symbol is called `main` in the output
//pub extern fn main(argc: i32, argv: *const *const u8) -> i32 {
pub extern fn main() {
	let pin = 16;
	let GPFSEL = GPFSEL();
	let GPSET = GPSET();
	let GPCLR = GPCLR();
	// Set as GPIO output
	GPFSEL.set_function_sel(pin, 1);
	
	loop {
		//let mut x = 0;
		GPCLR.set_pin(pin);
		for _ in 0..0x100000 { unsafe { asm!("nop"); } }
		GPSET.set_pin(pin);
		for _ in 0..0x100000 { unsafe { asm!("nop"); } }
	}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
	    loop {}
}

