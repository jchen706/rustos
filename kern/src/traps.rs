mod frame;
mod syndrome;
mod syscall;
use crate::console::kprintln;
use crate::shell;

pub mod irq;
pub use self::frame::TrapFrame;

use pi::interrupt::{Controller, Interrupt};
use pi::local_interrupt::{LocalController, LocalInterrupt};

use self::syndrome::Syndrome;
use self::syscall::handle_syscall;
use crate::vm::{PhysicalAddr, VirtualAddr};
use aarch64::*;
//use crate::IRQ;
use crate::percore;
use crate::traps::irq::IrqHandlerRegistry;

use crate::GLOBAL_IRQ;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern "C" fn handle_exception(info: Info, esr: u32, tf: &mut TrapFrame) {

    //kprintln!("Info Kind{:?}", info.kind);
    //kprintln!("Info Source {:?}", info.source);
    //kprintln!("Info ESR{:?}", esr);

    match info.kind {
        Kind::Synchronous => {
            match Syndrome::from(esr) {
                 Syndrome::Brk(num) => {
                    //kprintln!("brK {:?}", num);
                    tf.elr+=4;
                    shell::shell("debug$ ");
                    //kprintln!("returning {:?}", num);

                    return 

                 },
                 Syndrome::Svc(scall) => {
                    //kprintln!("svc {:?}", scall);
                    handle_syscall(scall, tf);
                    return

                 },
                 Syndrome::DataAbort {kind, level}=>{
                     
                            //kprintln!("DataAbort Level: {:?}", level);
                            unsafe {
                            panic!("Data Abort VirtualAddr: {:?}", VirtualAddr::from(FAR_EL1.get()));

                            }
                            aarch64::nop();

                    

                 },
                 _=> {

                     panic!(" Unimplemented Syndrome Information Kind: {:?}", info.kind);
                     aarch64::nop();

                 },
             }

        },
        Kind::Irq => {

            //kprintln!("irq {:?}", "interrupting");
            let core: u64 = 0;
            unsafe {
            let core =  MPIDR_EL1.get_value(MPIDR_EL1::Aff0);
            };


            if core == 0 {
                let control = Controller::new();
                let interrupt1 = Interrupt::iter();



                for each in interrupt1 {
                 //let m:()=each;
                    if control.is_pending(each) {
                        GLOBAL_IRQ.invoke(each, tf);

                    }
                }

            } else {
                let control1 = LocalController::new(core as usize);
                 let interrupt1 = LocalInterrupt::iter();



                for each in interrupt1 {
                 //let m:()=each;
                    if control1.is_pending(each) {
                        percore::local_irq().invoke(each, tf);

                    }
                }




            }



            



        },
        Kind::Fiq => {

        },
        Kind::SError => {

        }

    }

    // loop {
    //     kprintln!("brK {:?}", "endless loop");
    //     aarch64::nop();
    // }
    
    //unimplemented!("handle_exception")
}
