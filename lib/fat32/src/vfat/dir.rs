use alloc::string::String;
use alloc::vec::Vec;

use shim::const_assert_size;
use shim::ffi::OsStr;
use shim::io;
use shim::newioerr;

use crate::traits;
use crate::util::VecExt;
use crate::vfat::{Attributes, Date, Metadata, Time, Timestamp};
use crate::vfat::{Cluster, Entry, File, VFatHandle};

#[derive(Debug)]
pub struct Dir<HANDLE: VFatHandle> {
    pub vfat: HANDLE,
    // FIXME: Fill me in.
    
    //first cluster
    first_cluster: Cluster,






}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatRegularDirEntry {
    // FIXME: Fill me in.
    file_name: [u8;8],
    file_extension: [u8;3],
    attributes:Attributes,
    nt_reserve: u8,
    creation_time_tenths_seconds: u8,
    time: u16,
    date: u16,
    last_accessed: u16,
    high_entry_first_cluster_num: u16,
    last_modified_time: u16,
    last_modified_date:u16,
    low_en_first_cluster_num:u16,
    size_file: u32,
}


const_assert_size!(VFatRegularDirEntry, 32);


impl VFatRegularDirEntry {
    // add code here


    pub fn file_name(&self) -> String {

    }

    pub fn file_extension(&self)-> {

    }

    pub fn cluster_num(&self)-> Cluster {

    }

    pub fn size(&self)->u32 {
        self.size_file
    }



}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatLfnDirEntry {
    // FIXME: Fill me in.
    sequence_number:u8,
    name_chars:[u8;10],
    attributes:u8,
    type:u8,
    checksum:u8,
    second_name_chars:[u8;12],
    always_zero:u16,
    third_name_chars:u32,


}


const_assert_size!(VFatLfnDirEntry, 32);


impl VFatLfnDirEntry {



}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatUnknownDirEntry {
    // FIXME: Fill me in.
    id: u8,
    offset_space:[u8;10],
    attributes:u8,
    unused_space:[u8;20],
}

const_assert_size!(VFatUnknownDirEntry, 32);

pub union VFatDirEntry {
    unknown: VFatUnknownDirEntry,
    regular: VFatRegularDirEntry,
    long_filename: VFatLfnDirEntry,
}

 

impl<HANDLE: VFatHandle> Dir<HANDLE> {
    /// Finds the entry named `name` in `self` and returns it. Comparison is
    /// case-insensitive.
    ///
    /// # Errors
    ///
    /// If no entry with name `name` exists in `self`, an error of `NotFound` is
    /// returned.
    ///
    /// If `name` contains invalid UTF-8 characters, an error of `InvalidInput`
    /// is returned.
    pub fn find<P: AsRef<OsStr>>(&self, name: P) -> io::Result<Entry<HANDLE>> {
        unimplemented!("Dir::find()")
    }
}


//need to have an iterator

impl<HANDLE: VFatHandle> traits::Dir for Dir<HANDLE> {
    // FIXME: Implement `trait::Dir` for `Dir`.




}
