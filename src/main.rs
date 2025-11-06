#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(kodios::test_runner)]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
#[cfg(not(test))]
use kodios::println;
use kodios::{
    hlt_loop,
    memory::{self, BootInfoFrameAllocator},
};
use x86_64::{VirtAddr, structures::paging::Page};

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kodios::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kodios::init();

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    //let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page: Page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    hlt_loop();
}
