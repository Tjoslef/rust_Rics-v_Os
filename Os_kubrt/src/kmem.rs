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
    pub fn is_taken(&self) -> bool{
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

static mut KMEN_HEAD: *mut AllocList = null_mut();
 static mut KMEN_ALLOC: usize =0;
 static mut KMEN_PAGE_TABLE: *mut Table = null_mut();
pub fn get_head()-> *mut u8{
    unsafe{KMEN_HEAD as *mut u8}


}
pub fn get_page_table() -> *mut Table {
	unsafe { KMEM_PAGE_TABLE as *mut Table }
}

pub fn get_num_allocations() -> usize {
	unsafe { KMEM_ALLOC }
}
pub fn init() {
unsafe {
    let k_alloc = zalloc(64);
    assert!(!k_alloc.is_null());
    KMEN_ALLOC = 64;
    KMEN_ALLOC = k_alloc as *mut AllocList;
    (*KMEN_ALLOC).set_free();
    (*KMEM_HEAD).set_size(KMEM_ALLOC * PAGE_SIZE);
    KMEM_PAGE_TABLE = zalloc(1) as *mut Table;




    }



}
