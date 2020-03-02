use alloc::string::String;
use alloc::vec::Vec;

use shim::const_assert_size;
use shim::ffi::OsStr;
use shim::io;
use shim::newioerr;

use crate::traits;
use crate::util::VecExt;
use crate::vfat::{Attributes, Date, Metadata, Time, Timestamp};
use crate::vfat::{Cluster, Entry, File, VFatHandle, Error};

use core::char::decode_utf16;
use core::char::REPLACEMENT_CHARACTER;


#[derive(Debug)]
pub struct Dir<HANDLE: VFatHandle> {
    pub vfat: HANDLE,
    // FIXME: Fill me in.
    
    //first cluster
    pub start_cluster: Cluster,
    pub name: String,
    pub metadata: Metadata,






}

impl <HANDLE: VFatHandle> Dir<HANDLE> {
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
    attributes:u8,
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


    // pub fn file_name(&self) -> &str {
    //     let mut v:Vec<u8> = Vec::new();
    //     v.extend_from_slice(&self.file_name)
    //     let string_u = String::from_utf8(v).unwrap().trim_right();
    //     let mut e:Vec<u8> = Vec::new();
    //     e.extend_from_slice(&self.file_extension)
    //     let string_ext = String::from_utf8(e).unwrap.trim_right();

    //     if self.file_extension.len() > 0 {
    //         string_u.push_str(&".");
    //         string_u.push_str(string_ext);
    //         return string_u;
    //     }

    //     &string_u

    // }


    pub fn cluster_num(&self)-> Cluster {
    Cluster::from(((self.high_entry_first_cluster_num as u32) << 16) | self.low_en_first_cluster_num as u32)
    }

    pub fn size(&self)->u32 {
        self.size_file
    }

    pub fn is_dir(&self)->bool {
        Attributes::new(self.attributes).directory()
    }



}


#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VFatLfnDirEntry {
    // FIXME: Fill me in.
    sequence_number:u8,
    name_chars:[u16;5],
    attributes:u8,
    type1:u8,
    checksum:u8,
    second_name_chars:[u16;6],
    always_zero:u16,
    third_name_chars:[u16;2],


}


impl VFatLfnDirEntry {


    // pub fn file_name(&self) ->  {

    //     ///vec

    //     let mut v:Vec<u16> = Vec::new();
    //     v.extend_from_slice(&self.name_chars)
    //     let string_u = String::from_utf16(v).unwrap();
        
    //     if self.second_name_chars.len() > 0 {
    //         string_u.push_str(self.second_name_chars);
    //     } 

    //     if self.third_name_chars.len() > 0 {
    //         string_u.push_str(self.third_name_chars);
    //     }

    //     string_u

    // }


    // pub fn determine_termination(&self)->(usize, usize, usize) {
    //     let index1 = 0;
    //     let index2 = 0;
    //     let index3 = 0;

    //     for i in 0..self.name_chars.len() {
    //         if self.name_chars[i] == 0x00 || self.name_chars[i] == 0xFF {
    //             index1 = i;
    //         }
    //     }

    //     for i in 0..self.second_name_chars.len() {
    //         if self.name_chars[i] == 0x00 || self.name_chars[i] == 0xFF {
    //             index2 = i;
    //         }
    //     }

    //      for i in 0..self.third_name_chars.len() {
    //         if self.name_chars[i] == 0x00 || self.name_chars[i] == 0xFF {
    //             index3 = i;
    //         }
    //     }







    // }


    pub fn is_dir(&self)-> bool {
        Attributes::new(self.attributes).directory()
    }
    





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
        
        let string_name = match name.as_ref().to_str() {
            None=> return Err((io::Error::new(io::ErrorKind::InvalidInput, "Path is InvalidInput"))),
            Some(x)=> x,
        };


        use traits::Dir;
        use traits::Entry;
        for each in self.entries()? {


            if string_name.eq_ignore_ascii_case(each.name()) {
                return Ok(each);
            }
        }

        Err(io::Error::new(io::ErrorKind::NotFound,"Entry name not found"))


    }
}


//need to have an iterator struct

pub struct EntryIterator<HANDLE:VFatHandle> {
    current: usize,
    entries: Vec<VFatDirEntry>,
    vfat:HANDLE,

}

impl<HANDLE:VFatHandle> Iterator for EntryIterator<HANDLE> {
    type Item = Entry<HANDLE>;


    fn next(&mut self) -> Option<Self::Item> {


        //fetch entry
        let mut lfn_list = [0u16; 13*31];
        let mut is_lfn = false;


        loop {

            if(self.current > self.entries.len()) {
                return None
            } else {


                let entry = &self.entries[self.current];
                let unknown = unsafe {entry.unknown};

                //check for valid
                if unknown.is_del_unused() {
                    
                    self.current+=1;
                    continue;

                } else if unknown.is_end() {
                    return None;
                }


                if unknown.is_LFN() {

                    let lfn = unsafe {entry.long_filename};

                    //check sequence number 
                    let sequence = lfn.sequence_number & 0b00011111;

                    if sequence >= 0x01 && sequence <=0x1F {
                        let lfn_index = (sequence-1) as usize;
                        is_lfn = true;
                        lfn_list[lfn_index*13..lfn_index*13+5].copy_from_slice(&lfn.name_chars);
                        lfn_list[lfn_index*13+5..lfn_index*13+11].copy_from_slice(&lfn.second_name_chars);
                        lfn_list[lfn_index*13+11..lfn_index*13+13].copy_from_slice(&lfn.third_name_chars);

                    } else {
                    
                    }

                } else {

                    //regular 

                    let regular = unsafe{entry.regular};

                      

                    let string_name = if is_lfn {
                        // let decode_string = &(decode_utf16((lfn_list).iter().clone()))
                        // .map(|r| r.unwrap_or("")).collect::<Vec<_>>();
                        // let decode_string = decode_utf16(lfn_list[..].iter().cloned())
                        // .map(|r| r.map_err(|e| e.unpaired_surrogate())).collect::<Vec<_>>();

                        let decode_string = decode_utf16(lfn_list[..].iter().cloned())
                        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER) ).collect::<String>().replace(REPLACEMENT_CHARACTER, "");


                        decode_string
                       
                        
                    } else {

                        let mut name = regular.file_name.clone();


                        let mut last = 0;
                        for chars in 0..name.len() {

                            if name[chars] == 0x00 || name[chars] == 0x20 {

                                break;
                            } 
                            last+=1;

                        }

                        let mut valid_name = String::from_utf8(name[..last].to_vec()).unwrap();

                        let extension = String::from_utf8(regular.file_extension.to_vec()).unwrap();

                         if regular.file_extension.len() > 0 {
                          valid_name.push_str(&".");
                          valid_name.push_str(&extension);
                        }

                        valid_name

                    };

                    let cluster1 = Cluster::from(((regular.high_entry_first_cluster_num as u32) << 16) 
                        | regular.low_en_first_cluster_num as u32);






                    let metadata1 = Metadata {
                        created: Timestamp {
                            date: Date::new(regular.date).unwrap(),
                            time: Time::new(regular.time).unwrap(),
                        },
                        accessed: Timestamp {
                             date: Date::new(regular.last_accessed).unwrap(),
                            time: Time::new(0).unwrap(),
                        },
                        modified: Timestamp {
                             date: Date::new(regular.last_modified_date).unwrap(),
                            time: Time::new(regular.last_modified_time).unwrap(),
                        },
                        attributes: Attributes::new(regular.attributes),

                    };

                    


                    if regular.is_dir() {
                        return Some(Entry::Dir(Dir{
                            vfat:self.vfat.clone(),
                            name: string_name,
                            start_cluster: cluster1,
                            metadata: metadata1,
                        }));

                    } else {
                        return Some(Entry::File(File{
                            vfat:self.vfat.clone(),
                            name: string_name,
                            start_cluster: cluster1,
                            metadata: metadata1,
                            size: regular.size() as u64,
                            current_cluster: cluster1,
                            current_offset: 0 as u64,


                        }));
                    }
                }

            }
        }

    }


}





impl<HANDLE: VFatHandle> traits::Dir for Dir<HANDLE> {
    // FIXME: Implement `trait::Dir` for `Dir`.
    type Entry = Entry<HANDLE>;
    type Iter = EntryIterator<HANDLE>;


     /// Returns an interator over the entries in this directory.
    //println!("{:?}", "Here");
    fn entries(&self) -> io::Result<Self::Iter> {
        let mut vector = Vec::new();

        let entrys = self.vfat.lock(|vfat| vfat.read_chain(self.start_cluster, &mut vector));
        match entrys {
            Ok(_)=> {},
            Err(_) => 
            {return Err(io::Error::new(io::ErrorKind::InvalidData, "from dir entires looping through"))},
        }

        let entryiter = EntryIterator {
            current:0,
            vfat:self.vfat.clone(),
            entries: unsafe {vector.cast()}
        };






        Ok(entryiter)
    }



}
