mod linked_list;
mod util;

mod bin;
mod bump;

type AllocatorImpl = bin::Allocator;

#[cfg(test)]
mod tests;

use core::alloc::{GlobalAlloc, Layout};
use core::fmt;

use crate::mutex::Mutex;
use pi::atags::{Atag, Atags};
use crate::allocator::util::align_up;


/// `LocalAlloc` is an analogous trait to the standard library's `GlobalAlloc`,
/// but it takes `&mut self` in `alloc()` and `dealloc()`.
pub trait LocalAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout);
}

/// Thread-safe (locking) wrapper around a particular memory allocator.
pub struct Allocator(Mutex<Option<AllocatorImpl>>);

impl Allocator {
    /// Returns an uninitialized `Allocator`.
    ///
    /// The allocator must be initialized by calling `initialize()` before the
    /// first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {
        Allocator(Mutex::new(None))
    }

    /// Initializes the memory allocator.
    /// The caller should assure that the method is invoked only once during the
    /// kernel initialization.
    ///
    /// # Panics
    ///
    /// Panics if the system's memory map could not be retrieved.
    pub unsafe fn initialize(&self) {
        let (start, end) = memory_map().expect("failed to find memory map");
        info!("heap beg: {:x}, end: {:x}", start, end);
        *self.0.lock() = Some(AllocatorImpl::new(start, end));
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .as_mut()
            .expect("allocator uninitialized")
            .alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .as_mut()
            .expect("allocator uninitialized")
            .dealloc(ptr, layout);
    }
}

extern "C" {
    static __text_end: u8;
}

/// Returns the (start address, end address) of the available memory on this
/// system if it can be determined. If it cannot, `None` is returned.
///
/// This function is expected to return `Some` under all normal cirumstances.
pub fn memory_map() -> Option<(usize, usize)> {
    let page_size = 1 << 12;
    let binary_end = unsafe { (&__text_end as *const u8) as usize };
    
    //get the tags and find where memory ends as physical memory

    let tags = Atags::get();

    for each in tags {
        match each {
            Atag::Mem(x) => {
                let mut start = x.start as usize;
                let mut end: usize = start + x.size as usize;


                let y = align_up(binary_end, page_size);

                return Some((y, end));
            },
            Atag::None => {
                return None;
            },
            _=> {
                continue;
            }

        }
    } 

    return None;   

}

impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.lock().as_mut() {
            Some(ref alloc) => write!(f, "{:?}", alloc)?,
            None => write!(f, "Not yet initialized")?,
        }
        Ok(())
    }
}
