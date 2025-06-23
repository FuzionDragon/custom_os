#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{info}");
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  kernel::test_panic_handler(info);

  loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  println!("Hello World!");
  println!("You are a nerd! {}", 1);
  serial_println!("Serial output test");

  kernel::init();
//  x86_64::instructions::interrupts::int3();

//  unsafe {
//    *(0xdeadbeef as *mut u8) = 42;
//  };

//  fn stack_overflow() {
//    stack_overflow(); // for each recursion, the return address is pushed
//  }
//
//  stack_overflow();

  #[cfg(test)]
  test_main();

  println!("No crash happened!");
  
  loop {}
}
