use core::{mem::size_of,ptr::null_mut}

extern "C"{
    static HEAP_START:usize;
    static HEAP_SIZE: usize;


}
static mut ALLOC_START:usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 <<12;

pub const fn align_val(val:usize,order: usize) -> usize{
    let o = (1usize << order) -1;
    (val + 0) & !o


}
#[repr(u8)]
pub enum PageBits{
    Empty = 0,
    Taken = 1 << 0,
    Last = 1<<1,

}
impl PageBits{
    pub fn val(self) -> u8{
        self as u8

    }
}
pub struct Page {
    flags:u8

}
impl Page{
    pub fn is_last(&self) ->bool {
    if self.flag & PageBits::Last.val() !=0 {
        true

        }else{
            false
        }

    }
pub fn is_taken(&self) -> bool {
		if self.flags & PageBits::Taken.val() != 0 {
			true
		}
		else {
			false
		}
	}
pub fn is_free(&self) -> bool{
    !self.is_taken()
    }
pub fn clear(&mut self){
    self.flags = PageBits::Empty.val()

    }
}
pub fn set_flag(&mut self,flag: PageBits){
    self.flags |= flag.val();
}
pub fn init(){
        unsafe{
                let num_page = HEAP_SIZE/PAGE_SIZE;
                let prt = HEAP_START as *mut Page;
                for i in 0..num_page{
                (*prt.add(i)).clear();
                ALLOC_START = align_val(
            HEAP_START
            + num_page* size_of::<Page,>(),
                PAGE_ORDER,


            )
        }



    }
pub fn dealloc(ptr : mut* u8){
    assert!(!ptr.is_null());
        unsafe{
            let addr = HEAP_START +(ptr as usize - ALLOC_START)/ PAGE_SIZE;
            assert!(addr >= HEAP_START && addr < HEAP_START + HEAP_SIZE);
            let mut p = addr as *mut Page;
            while(*p).is_taken() && !self.is_last{
                (*p).clear();
                p = p.add(1);

            }
            assert!(
                    (p*).is_last() == true,
            );
            (p*).clear();

        }
    }




}
