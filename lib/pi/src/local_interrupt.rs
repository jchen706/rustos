use core::time::Duration;

use volatile::prelude::*;
use volatile::Volatile;

const INT_BASE: usize = 0x40000000;




use aarch64::regs::CNTP_TVAL_EL0;
use aarch64::CNTFRQ_EL0;
use aarch64::CNTP_CTL_EL0;



/// Core interrupt sources (QA7: 4.10)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LocalInterrupt {
    // Lab 5 1.C
    // FIXME: please fill in the definition

    Cntpsirq = 0,
    Cntpnsirq =1,
    Cnthpirq = 2,
    Cntvirq = 3,
    Mailbox0 = 4,
    Mailbox1= 5,
    Mailbox2 = 6,
    Mailbox3 = 7,
    GpuIrq = 8,
    PmuIrq = 9,
    AxiOutIrq = 10,
    LocalTimerIrq = 11,
 
}

impl LocalInterrupt {
    pub const MAX: usize = 12;

    pub fn iter() -> impl Iterator<Item = LocalInterrupt> {
        (0..LocalInterrupt::MAX).map(|n| LocalInterrupt::from(n))
    }
}

impl From<usize> for LocalInterrupt {
    fn from(irq: usize) -> LocalInterrupt {
        // Lab 5 1.C
        //unimplemented!("LocalInterrupt")
        use LocalInterrupt::*;
          match irq {
            0 => Cntpsirq, 
            1 => Cntpnsirq, 
            2 => Cnthpirq,
            3 => Cntvirq,
            4 => Mailbox0, 
            5 => Mailbox1, 
            6 => Mailbox2, 
            7 => Mailbox3,
            8 => GpuIrq, 
            9 => PmuIrq, 
           10 => AxiOutIrq, 
           11 => LocalTimerIrq, 
           _=> panic!(" LocalInterrupt Not Matched {:?}", irq),
          }
    }
}

/// BCM2837 Local Peripheral Registers (QA7: Chapter 4)
#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    // Lab 5 1.C
    // FIXME: please fill in the definition

    control_reg: Volatile<u32>,
    unused_1: Volatile<u32>,
    core_timer_prescaler: Volatile<u32>,
    gpu_interrupts_routing: Volatile<u32>,
    pmi_routing_set: Volatile<u32>,
    pmi_routing_clear: Volatile<u32>,
    unused_2: Volatile<u32>,

    core_timer_access_low: Volatile<u32>,
    core_timer_access_high: Volatile<u32>,
    local_interrupts_0: Volatile<u32>,
    unused_3: Volatile<u32>,


    axi_outstanding_counters: Volatile<u32>,
    axi_outstanding_irq: Volatile<u32>,
    local_timer_control_status: Volatile<u32>,
    local_timer_write_flags: Volatile<u32>,
    unused_4: Volatile<u32>,

    core_timer_interupt_c: [Volatile<u32>; 4],
    core_mail_interupt_c: [Volatile<u32>; 4],
    core_irq_source: [Volatile<u32>; 4],    
    core_fiq_source: [Volatile<u32>; 4],




}

pub struct LocalController {
    core: usize,
    registers: &'static mut Registers,
}

impl LocalController {
    /// Returns a new handle to the interrupt controller.
    pub fn new(core: usize) -> LocalController {
        LocalController {
            core: core,
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    pub fn enable_local_timer(&mut self) {
        // Lab 5 1.C
        //unimplemented!("LocalInterrupt")
        unsafe {
            CNTP_CTL_EL0.set( CNTP_CTL_EL0.get()| CNTP_CTL_EL0::ENABLE);
        }

        self.registers.core_irq_source[self.core].write(1 << 1);


    }

    pub fn is_pending(&self, int: LocalInterrupt) -> bool {
        // Lab 5 1.C
        //unimplemented!("LocalInterrupt")
        self.registers.core_irq_source[self.core].has_mask(1<< int as usize)
    }

    pub fn tick_in(&mut self, t: Duration) {
        // Lab 5 1.C
        // See timer: 3.1 to 3.3
        //unimplemented!("LocalInterrupt")
        let freq = unsafe {CNTFRQ_EL0.get()};
        let timer_ticks = (freq / 1000) * t.as_millis() as u64;

        unsafe {
            CNTP_TVAL_EL0.set(CNTP_TVAL_EL0::TVAL & timer_ticks);
           
        }



    }
}

pub fn local_tick_in(core: usize, t: Duration) {
    LocalController::new(core).tick_in(t);
}
