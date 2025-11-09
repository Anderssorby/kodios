#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(kodios::test_runner)]

extern crate alloc;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
#[cfg(not(test))]
use kodios::println;
use kodios::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    serial_println,
    task::{Task, executor::Executor, keyboard},
};
use x86_64::VirtAddr;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kodios::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use kodios::hlt_loop;
    println!("{}", info);
    hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    serial_println!("async number: {}", number);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kodios::init();

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
