use crate::page::{align_val, zalloc, Table, PAGE_SIZE};
use core::{mem::size_of, ptr::null_mut};use crate::page::{align_val, zalloc, Table, PAGE_SIZE};
enum AllocListFlags{
Taken = 1<<63,
}
impl AllocListFlags{
   pub fn val(self) -> usize {
		self as usize
	}
}
struct AllocList {
	pub flags_size: usize,
}
impl AllocList{
    pub fn is taken(&self) -> bool{
    self.flags_size & AllocListFlags::Taken.val() !=0
    }
    pub fn is_free(&self) -> bool {
		!self.is_taken()
	}

	pub fn set_taken(&mut self) {
		self.flags_size |= AllocListFlags::Taken.val();
	}

	pub fn set_free(&mut self) {
		self.flags_size &= !AllocListFlags::Taken.val();
	}

	pub fn set_size(&mut self, sz: usize) {
		let k = self.is_taken();
		self.flags_size = sz & !AllocListFlags::Taken.val();
		if k {
			self.flags_size |= AllocListFlags::Taken.val();
		}
	}

	pub fn get_size(&self) -> usize {
		self.flags_size & !AllocListFlags::Taken.val()
	}
}

unsafe static mut KMEN_HEAD: *mut AllocList = null_mut();
unsafe static mut KMEN_ALLOC: usize =0;
unsafe static mut KMEN_PAGE_TABLE: *mut Table = null_mut();

