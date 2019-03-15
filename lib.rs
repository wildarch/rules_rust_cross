#![no_std]

use core::panic::PanicInfo;
use hello_macro::HelloMacro;

trait HelloMacro {
    fn hello_macro() -> &'static str;
}

#[derive(HelloMacro)]
struct Daan;

#[cfg(not(test))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

pub fn test() -> &'static str {
    Daan::hello_macro()
}

#[cfg(test)]
mod tests {
    extern crate std;

    #[test]
    fn test() {
        use super::{Daan, HelloMacro};
        assert_eq!(Daan::hello_macro(), "Hello, Macro! My name is Daan");
    }
}
