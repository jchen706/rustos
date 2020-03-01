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

use core::char::decode_utf16;

#[derive(Debug)]
pub struct Dir<HANDLE: VFatHandle> {
    pub vfat: HANDLE,
    // FIXME: Fill me in.
    
    //first cluster
    pub start_cluster: Cluster,
    pub name: String,
    pub metadata: Metadata,






}

impl Dir {
    pub fn name(&self)-> &str {
        &self.name
    }

    pub fn metadata(&self)-> &Metadata {
        &self.metadata
    }
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
        let mut v:Vec<u8> = Vec::new();
        v.extend_from_slice(&self.file_name)
        let string_u = String::from_utf8(v).unwrap();

    }


    pub fn cluster_num(&self)-> Cluster {
    Cluster::from(((self.high_entry_first_cluster_num as u32) << 16) | self.low_en_first_cluster_num)
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




#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatUnknownDirEntry {
    // FIXME: Fill me in.
    id: u8,
    offset_space:[u8;10],
    attributes:u8,
    unused_space:[u8;20],
}


impl VFatUnknownDirEntry {


    pub fn is_LFN(&self)-> bool {
        self.attributes == 0x0F
    }

    pub fn is_end(&self)-> bool {
        self.id == 0x00
    }

    pub fn is_del_unused(&self)->bool{
        self.id == 0xE5
    }

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


//need to have an iterator struct

pub struct EntryIterator {
    current: usize,
    entrys: Vec<VFatDirEntry>,
    vfat:HANDLE,

}

impl Iterator for EntryIterator {
    type Item = Entry<HANDLE>;


    fn next(&mut self) -> Option<Self::Item> {
        





    }




}





impl<HANDLE: VFatHandle> traits::Dir for Dir<HANDLE> {
    // FIXME: Implement `trait::Dir` for `Dir`.
    type Entry = Entry<HANDLE>;
    type Iter = EntryIteratorHelper;


     /// Returns an interator over the entries in this directory.
    fn entries(&self) -> io::Result<Self::Iter> {
        panic!("Dummy")
    }



}
