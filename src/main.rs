#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(kodios::test_runner)]


use core::panic::PanicInfo;
use kodios::hlt_loop;
#[cfg(not(test))]
use kodios::println;

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

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kodios::init();
    
    #[cfg(test)]
    test_main();

    hlt_loop();
}

