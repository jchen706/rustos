#![feature(asm)]
#![feature(global_asm)]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]


#![allow(dead_code)]
use rand_core::{RngCore, Error, impls};

#[cfg(not(test))]
mod init;

use rand::{Rng};


const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;


#[derive(Default)]
struct RdRand;


// const RNG_CTRL   : *mut u32 = (GPIO_BASE+0x00104000) as *mut u32;
// const RNG_STATUS  : *mut u32 =  (GPIO_BASE+0x00104004) as *mut u32;
// const RNG_DATA     : *mut u32 = (GPIO_BASE+0x00104008) as *mut u32;
// const RNG_INT_MASK  : *mut u32 = (GPIO_BASE+0x00104010) as *mut u32;

// fn init_rand() {
//     RNG_STATUS=0x40000;
//     // mask interrupt
//     RNG_INT_MASK|=1;
//     // enable
//     RNG_CTRL|=1;
//     // wait for gaining some entropy
//     loop { if(!((RNG_STATUS)>>24))) { 
//      unsafe { asm!("nop" :::: "volatile");} };

// }

// fn gen_range (&mut self, min : usize, max : usize) -> usize {
//     RNG_DATA % (max-min) + min
// }



// impl RngCore for RdRand {
//     fn next_u32(&mut self) -> u32 {
//            self.next_u64() as u32

//     }

//     fn next_u64(&mut self) -> u64 {
//          self.gen_range(0,1000) as u64
		

//     }

//     fn fill_bytes(&mut self, dest: &mut [u8]) {
//         impls::fill_bytes_via_next(self, dest)
//     }

//     fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
//         Ok(self.fill_bytes(dest))
//     }
// }


#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

unsafe fn kmain() -> ! {
    // FIXME: STEP 1: Set GPIO Pin 16 as output.
    GPIO_FSEL1.write_volatile(GPIO_FSEL1.read_volatile()|1<<18);

    // FIXME: STEP 2: Continuously set and clear GPIO 16.
    //let mut rng: RdRand = Default::default();

    loop {
	GPIO_SET0.write_volatile(GPIO_SET0.read_volatile()|1<<16);
	spin_sleep_ms(rng.gen_range(0, 1000));

	GPIO_CLR0.write_volatile(GPIO_CLR.read_volatile()|1<<16);
	spin_sleep_ms(rng.gen_range(0, 1000));

	}
}
