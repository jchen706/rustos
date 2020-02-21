#![feature(panic_info_message)]
use core::panic::PanicInfo;


use crate::console::kprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    if let Some(location) = _info.location() {
        kprintln!("Panic occurred in file {}", location.file());
        kprintln!("Panic occurred on line {}", location.line());
    } else {
        kprintln!("{:?}", _info.message());
    }



    loop {        
    }
}
