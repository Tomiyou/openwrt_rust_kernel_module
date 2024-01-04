use core::mem;
use core::panic::PanicInfo;
pub use crate::bindings;

extern "C" {
    fn bug_helper() -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        bug_helper();
    }
}
