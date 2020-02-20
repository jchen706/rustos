#![feature(panic_info_message)]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    if let Some(location) = _info.location() {
        kprintln!("Panic occurred in file '{}' at line {}", location.file(), location.line);
    } else {
        kprintln!("{:?}", _info.message());
    }



    loop {        
    }
}
