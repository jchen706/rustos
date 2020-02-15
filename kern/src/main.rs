#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

pub mod console;
pub mod mutex;
pub mod shell;

use console::kprintln;
use console::kprint;

// FIXME: You need to add dependencies here to
// test your drivers (Phase 2). Add them as needed.
use core::fmt::Write;

use pi::timer::spin_sleep;
use core::time::Duration;
use pi::gpio::Gpio;
use pi::uart::MiniUart;

use crate::shell::shell;



// const GPIO_BASE: usize = 0x3F000000 + 0x200000;
//
// const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
// const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
// const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;




fn kmain() -> ! {
    // FIXME: Start the shell.
    //
    //let mut test1 = Gpio::new(16).into_output();


    //Gpio::new(15).into_output();
    //GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile() | (1<<18));
    //let mut m = MiniUart::new();

    loop {
        //GPIO_SET0.write_volatile(GPIO_SET0.read_volatile() | (1<<16));
        // test1.set();
        // spin_sleep(Duration::new(1,0));
        // test1.clear();
        // GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (1<<16));
        // spin_sleep(Duration::new(1,0));
        //let byte = m.read_byte();
        //kprintln!("{} receive", byte);
        //kprint!("good");
        //m.write_byte(byte);
        //spin_sleep(Duration::new(6,0));
        //kprintln!("{}","Start");
        shell(">");
        


    }



}
