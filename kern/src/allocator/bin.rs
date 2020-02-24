use core::alloc::Layout;
use core::fmt;
use core::ptr;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;

/// A simple allocator that allocates based on size classes.
///   bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   ...
///   bin 29 (2^22 bytes): handles allocations in (2^31, 2^32]
///   
///   map_to_bin(size) -> k
///   


//plan 
// 1. we need to have start and end nodes 
//32 bins // less calculation
// n from 3 to k 
//represent the free memory


//global memory is from memory map
pub struct Allocator {
    list:[LinkedList; 30],


    global_start: usize,
    global_end: usize,

    //what happens when a bin is exhausted
    total_size:usize


}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {

       let size = start - end;


       let equal_size = size / 30;
       
       let mut list1:[LinkedList; 30] = [LinkedList::new(); 30];

       unsafe {
            list1[0].push(start as *mut usize);
       }
       let mut dummy_start = start;

       unsafe {
            for i in 1..30 {
                dummy_start += equal_size;
                list1[i].push(dummy_start as *mut usize); 
            }
        }

       Allocator {
        list: list1,
        global_start: start,
        global_end: end,
        total_size: size

       }
       
        
    }


    pub fn map_to_bin(mut size: usize) -> usize {

        let mut count = 0;

        if size <= 8 {
             0
        } else {

            while size != 0 {
                size = size /2;
                count+=1;
            }

            count-1

        }




    }
}

impl LocalAlloc for Allocator {
    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning null pointer (`core::ptr::null_mut`)
    /// indicates that either memory is exhausted
    /// or `layout` does not meet this allocator's
    /// size or alignment constraints.
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        
        if (self.total_size < layout.size()) {
            return core::ptr::null_mut() as *mut u8;
        }



        //determine bin from size:
        let bin_num = Allocator::map_to_bin(layout.size());

      
        let address =  self.list[bin_num].pop();

        //align the current pointer
        let align_address = align_up(address.unwrap() as usize, layout.align());

        //find the end of the block of memory
        let end_address = align_address.saturating_add(layout.size());

        //end can't be more than self.end
       
        //start of the new block
        //let pointer: *mut u8 = (end - layout.size()) as *mut u8;

        //self.current = end;

        println!("{:?}", address);

        address.unwrap() as *mut u8







    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        unimplemented!("bin allocator")
    }
}

// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_> )-> fmt::Result {
        Ok(())
    }
}