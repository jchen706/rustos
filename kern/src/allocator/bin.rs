use core::alloc::Layout;
use core::fmt;
use core::ptr;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;


/// A simple allocator that allocates based on size classes.
///   bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   ...5  , 6   , 7 , 8 , 9 , 10
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
    relative_start: usize,

    //what happens when a bin is exhausted
    total_size:usize


}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {

        let size = end - start;
        
        let mut list1:[LinkedList; 30] = [LinkedList::new(); 30];

       // unsafe {
       //      list1[0].push(start as *mut usize);
       // }
       // let mut dummy_start = start;

       // unsafe {
       //      for i in 1..30 {
       //          dummy_start += equal_size;
       //          list1[i].push(dummy_start as *mut usize); 
       //      }
       //  }

       Allocator {
        list: list1,
        global_start: start,
        relative_start: start,
        global_end: end,
        total_size: size

       }
       
        
    }


    pub fn map_to_bin(mut size: usize) -> (usize, usize) {


        let mut count = 0;
        println!("{:?}", size );

        let mut next_power = size.next_power_of_two();

        while next_power > 0 {
            count+=1;
            next_power = next_power >> 1;
        }

        count = count -1;

        let mut bin_number = 0;

        if count <= 3 {
            bin_number = 0;
        } else {
            bin_number = count - 3;
        }

        println!("{:?}", count);
        (bin_number, 1<<(count))



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





        //turn layout.size to the closest power of 2

        //determine bin from size:
        //

        //max of layout align and smallest bin possible 



        //size = 256 
        /// alignment is 128

        let mut max_size =  layout.size();

        let mut change_size = false;
        if layout.size() < layout.align() {
             max_size = layout.align();
             println!("{:?}", "align changed" );
             change_size = true;
        }
        

        let (bin_num , nearest_size) = Allocator::map_to_bin(max_size);

        // if change_size {
        //     let mut x1 = Allocator::map_to_bin(max_size);

        // } 

        //check if bin is empty

        let is_empty_list = self.list[bin_num].is_empty();

        //println!("{:?}", is_empty_list);


        if is_empty_list {

            // if self.relative_start < self.global_start || self.relative_start > self.global_end {
            //     return core::ptr::null_mut() as *mut u8;
            // } else {



                //loop through all the bins
                //if the bin not empty merge the size





                let mut start_addr = align_up(self.relative_start, layout.align());
               

                //if change_align {
                  //  start_addr = align_up(self.relative_start, nearest_size);
                //}

                if start_addr > self.global_end {
                   return core::ptr::null_mut() as *mut u8;
                }


         

                let end_addr = start_addr.saturating_add(nearest_size);

                self.relative_start = end_addr;

                //println!("GLOBAL bin: {0}, nearest size: {1}, size: {2}, align: {6}, relative_start: {3}, start_addr: {4}, end_addr: {5}", bin_num, nearest_size, layout.size(), self.relative_start, start_addr, end_addr, layout.align());


                return start_addr as *mut u8;


            //}

        } else {


            //allocation problem here 
            let start_address: *mut usize = self.list[bin_num].pop().unwrap();

            let start_addr = start_address as usize;


            //println!("POP OFF bin: {0}, nearest size: {1}, size: {2}, align: {5}, relative_start: {3}, start_addr: {4}", bin_num, nearest_size, layout.size(), self.relative_start,start_addr ,layout.align());



            return start_address as *mut u8;






        }

    

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


        let mut max_size =  layout.size();
        if layout.size() < layout.align() {
            max_size = layout.align();
        }

        //find max of size and alisgn
        let (bin_num , nearest_size) = Allocator::map_to_bin(max_size);


        //println!("PUSH INTO bin: {0}, nearest size: {1}, size: {2}, align: {4}, relative_start: {3}", bin_num, nearest_size, layout.size(), self.relative_start, layout.align());

        



        self.list[bin_num].push(ptr as *mut usize);



    }
}

// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_> )-> fmt::Result {
        writeln!(f, "Bin Allocator Test")?;

        writeln!(f, "self.global_start: {}", self.global_start)?;
        writeln!(f, "self.global_end: {}", self.global_end)?;
        writeln!(f, "self.relative_start: {}", self.relative_start)?;
        writeln!(f, "self.total_size: {}", self.total_size)?;

      




        Ok(())
    }
}