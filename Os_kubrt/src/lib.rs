
#![no_std]
#![feature(panic_info_message,asm)]

use core::arch::asm;
use core::usize;

use self::page::{Table, PAGE_SIZE};
use self::uart::Uart;
mod uart;
mod page;
mod kmem;
// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
	($($args:tt)+) => ({    use core::fmt::Write;
			let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);

	});
}
#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[unsafe(no_mangle)]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	print!("Aborting: ");
	if let Some(p) = info.location() {
        let message_unwrappe = info.message();
		println!(
					"line {}, file {}: {}",
					p.line(),
					p.file(),
					message_unwrappe
		);
	}
	else {
		println!("no information available.");
	}
	abort();
}
#[unsafe(no_mangle)]
extern "C"
fn abort() -> ! {
	loop {
		unsafe {
			asm!("wfi");
		}
	}
}
unsafe extern "C" {
    static TEXT_START: usize;
	static TEXT_END: usize;
	static DATA_START: usize;
	static DATA_END: usize;
	static RODATA_START: usize;
	static RODATA_END: usize;
	static BSS_START: usize;
	static BSS_END: usize;
	static KERNEL_STACK_START: usize;
	static KERNEL_STACK_END: usize;
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
	static mut KERNEL_TABLE: usize;
}
pub fn id_map_range(root: &mut page::Table,
                    start: usize,
                    end: usize,
                    bits: i64)
{
	let mut memaddr = start & !(page::PAGE_SIZE - 1);
	let num_kb_pages = (page::align_val(end, 12)
	                 - memaddr)
	                / page::PAGE_SIZE;

	for _ in 0..num_kb_pages {
		page::map(root, memaddr, memaddr, bits, 0);
		memaddr += 1 << 12;
	}
}
// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C" fn kinit() -> usize {
	// We created kinit, which runs in super-duper mode
	// 3 called "machine mode".
	// The job of kinit() is to get us into supervisor mode
	// as soon as possible.
	// Interrupts are disabled for the duration of kinit()
	uart::Uart::new(0x1000_0000).init();
	page::init();
	kmem::init();

	// Map heap allocations
	let root_ptr = kmem::get_page_table();
	let root_u = root_ptr as usize;
	let mut root = unsafe { root_ptr.as_mut().unwrap() };
	let kheap_head = kmem::get_head() as usize;
	let total_pages = kmem::get_num_allocations();
	println!();
	println!();
}
pub fn id_map_range(root: &mut page::Table,
start:usize,
end: usize,
bits: i64,

){
    let mud memaddr = start & !(page:PAGE_SIZE -1);
    let num_kb_pages = (page::align_val(end,12)- memaddr)
		/ page::PAGE_SIZE;
    for _ in 0..num_kb_pages{
    page::map(root, memaddr, memaddr, bits,0);
    memaddr += 1 <<12;
    }

}
unsafe extern "C" fn kinit() -> usize{
 uart::Uart::new(0x1000_0000).init();
    page::init();
    kmen::init();

    let root_ptr = kmen::get_page_table();
    let root_u = root_ptr as usize;
    let mut root = unsafe{root_ptr.as_mut().unwrap()};
    let kheap_heap = kmen::get_head() as usize;
    let total_pages = kmen::get_num_allocation();
    println!();
	println!();
	unsafe {
		println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
		println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
		println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
		println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
		println!("STACK:  0x{:x} -> 0x{:x}", KERNEL_STACK_START, KERNEL_STACK_END);
		println!("HEAP:   0x{:x} -> 0x{:x}", kheap_head, kheap_head + total_pages * 4096);
	}
    id_map_range(&mut root, kheap_heap, kheap_heap +total_pages * 4096, page::EntryBits::ReadWrite.val());
     unsafe {
		// Map heap descriptors
		let num_pages = HEAP_SIZE / page::PAGE_SIZE;
		id_map_range(&mut root,
					 HEAP_START,
					 HEAP_START + num_pages,
					 page::EntryBits::ReadWrite.val()
		);
		// Map executable section
		id_map_range(
		             &mut root,
		             TEXT_START,
		             TEXT_END,
		             page::EntryBits::ReadExecute.val(),
		);
		// Map rodata section
		// We put the ROdata section into the text section, so they can
		// potentially overlap however, we only care that it's read
		// only.
		id_map_range(
		             &mut root,
		             RODATA_START,
		             RODATA_END,
		             page::EntryBits::ReadExecute.val(),
		);
		// Map data section
		id_map_range(
		             &mut root,
		             DATA_START,
		             DATA_END,
		             page::EntryBits::ReadWrite.val(),
		);
		// Map bss section
		id_map_range(
		             &mut root,
		             BSS_START,
		             BSS_END,
		             page::EntryBits::ReadWrite.val(),
		);
		// Map kernel stack
		id_map_range(
		             &mut root,
		             KERNEL_STACK_START,
		             KERNEL_STACK_END,
		             page::EntryBits::ReadWrite.val(),
		);
	}
page::map(
	          &mut root,
	          0x1000_0000,
	          0x1000_0000,
	          page::EntryBits::ReadWrite.val(),
			  0
	);

	// CLINT
	//  -> MSIP
	page::map(
	          &mut root,
	          0x0200_0000,
	          0x0200_0000,
	          page::EntryBits::ReadWrite.val(),
			  0
	);
	//  -> MTIMECMP
	page::map(
	          &mut root,
	          0x0200_b000,
	          0x0200_b000,
	          page::EntryBits::ReadWrite.val(),
			  0
	);
	//  -> MTIME
	page::map(
	          &mut root,
	          0x0200_c000,
	          0x0200_c000,
	          page::EntryBits::ReadWrite.val(),
			  0
	);
	// PLIC
	id_map_range(
	             &mut root,
	             0x0c00_0000,
	             0x0c00_2000,
	             page::EntryBits::ReadWrite.val(),
	);
	id_map_range(
	             &mut root,
	             0x0c20_0000,
	             0x0c20_8000,
	             page::EntryBits::ReadWrite.val(),
	);
    page::print_page_allocation();
    let p = 0x8005_7000 as usize;
    let m = page::virt_to_phys(&root,p).unwrap(0)
    println!("Walk 0x{:x} = 0x{:x}", p, m);
    unsafe{
    KERNEL_TABLE =root_u;


    }
(root_u >> 12)  | (8 << 60)
}

fn kmain(){
let mut my_uart = uart::Uart::new(0x1000_0000);
	my_uart.init();
println!("hello word from OS");

loop{
    if let Some(c) = my_uart.get(){
            match c {
                        8 => {
                            print!("{}{}",8 as char,8 as char);

                },
                10 | 13 =>{
                    println!();
                },
                0x1b =>{
                if let Some(next_byte) = my_uart.get(){
                    if next_byte == 91{
                        if let Some(b) = my_uart.get() {
                            match b as char {
                                   'A' => {
										  println!("That's the up arrow!");
									  },
									'B' => {
										  println!("That's the down arrow!");
									  },
									'C' => {
										  println!("That's the right arrow!");
									  },
									'D' => {
										  println!("That's the left arrow!");
									  },
									  _ => {
										  println!("That's something else.....");


                                }



                            };


                        };


                    }



                }
                }
                _=>{
                print!("{}",c as char)


            },


            }
}




    }
    }
