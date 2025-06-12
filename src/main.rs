#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

static MESSAGE: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  // 0xb8000 happens to be the buffer location
  // Our pointer
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in MESSAGE.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      // 0xb is a colour byte for light cyan
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }
  
  loop {}
}
