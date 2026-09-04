// All logic is implemented in main.c for minimal binary size.
// The real entry point is `main` provided by the C code.
#![no_main]
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}