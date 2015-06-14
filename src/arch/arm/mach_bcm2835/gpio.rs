
macro_rules! PBASE {() => (0x3F000000u32);}

pub struct GPFSELType {
	pub sel: [u32; 6],
}

impl GPFSELType {
	pub fn get_function_sel(&self, pin:u8) -> u8 {
		let index = pin/10;
		match pin {
			0...53 => unsafe {
				(self.sel.get_unchecked(index as usize) >> 3*(pin-(10*index))) as u8 & 0x7 
			},
			_ => 0
		}

	}
	pub fn set_function_sel(&mut self, pin:u8, sel:u8) {
		let index = pin/10;
		let offset = 3*(pin%10);
		unsafe {
			*self.sel.get_unchecked_mut(index as usize) &= ! (0x7u32 << offset);
			*self.sel.get_unchecked_mut(index as usize) |= (sel as u32) << offset;
		}
	}
}
pub struct GPCommonType {
	pub set: [u32; 2],
}

impl GPCommonType {
	pub fn set_pin (&mut self, pin:u8) {
//		self.set[(pin/32) as usize] = 1 << pin;
		unsafe { *self.set.get_unchecked_mut((pin/32) as usize) = 1 << pin; }
	}
}

#[inline(always)]
pub fn GPFSEL() -> &'static mut GPFSELType {unsafe {&mut *((PBASE!() + 0x200000u32) as *mut GPFSELType)}}
#[inline(always)]
pub fn GPSET() -> &'static mut GPCommonType {unsafe {&mut *((PBASE!() + 0x20001Cu32) as *mut GPCommonType)}}
#[inline(always)]
pub fn GPCLR() -> &'static mut GPCommonType {unsafe {&mut *((PBASE!() + 0x200028u32) as *mut GPCommonType)}}

