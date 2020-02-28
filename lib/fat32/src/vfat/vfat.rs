use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::size_of;

use alloc::vec::Vec;

use shim::io;
use shim::ioerr;
use shim::newioerr;
use shim::path;
use shim::path::Path;

use crate::mbr::MasterBootRecord;
use crate::traits::{BlockDevice, FileSystem};
use crate::util::SliceExt;
use crate::vfat::{BiosParameterBlock, CachedPartition, Partition};
use crate::vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Status};

/// A generic trait that handles a critical section as a closure
pub trait VFatHandle: Clone + Debug + Send + Sync {
    fn new(val: VFat<Self>) -> Self;
    fn lock<R>(&self, f: impl FnOnce(&mut VFat<Self>) -> R) -> R;
}

#[derive(Debug)]
pub struct VFat<HANDLE: VFatHandle> {
    phantom: PhantomData<HANDLE>,
    device: CachedPartition,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    rootdir_cluster: Cluster,
}

impl<HANDLE: VFatHandle> VFat<HANDLE> {
    pub fn from<T>(mut device: T) -> Result<HANDLE, Error>
    where
        T: BlockDevice + 'static,
    {

        //why phantom data

        //bytes per sector: bpb
        //sector per cluster: bpb
        // sectors per fat: bpb
        // 

        let mbr = MasterBootRecord::from(&mut device)?;

        //get the bpb partition adrress
        let bpb = mbr.partition_table[0];

        if !bpb.is_fat32() {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")));
        }



        //relative offset, offset in sectors from the start of disk to the partition

        let bpb_sector = bpb.relative_sector as u64;

        let ebpb = BiosParameterBlock::from(&mut device, bpb_sector);


        let bytes_per_sector = ebpb.bytes_per_sector;
        let rootdir = ebpb.root_cluster;
        let sect_per_fat = ebpb.size_FAT_sectors;
        let sect_per_cluster = ebpb.sector_per_cluster;

        //fat start sector

        //offset of fat from ebpb
        let number_of_reserve_sec = ebpb.reserve_sector;
        let fat_start = bpb_sector as u64 + number_of_reserve_sec as u64;

         


        //data start sector 

        //number of fats * size of fats + fat offset = first address of data region
        //sector per fact *  32 bytes of 16 bytes + fat_sart
        let data_start = ebpb.num_fat as u64 * sect_per_fat as u64 + fat_start;

        //number of sectors in parition = sectors of fat + sectors of clutster
        let partition1 = Partition {
            start: bpb_sector as u64,
            num_sectors: sect_per_fat as u64 * ebpb.num_fat as u64 + sect_per_cluster as u64,
            sector_size: bytes_per_sector as u64,

        }


        let cache_partition = CachedPartition::new(device, partition1);


        Ok(VFat {
            device: partition1,
            bytes_per_sector: bytes_per_sector,
            sectors_per_cluster: sect_per_cluster,
            sectors_per_fat: sect_per_fat,
            fat_start_sector: fat_start,
            data_start_sector: data_start as u64,
            rootdir_cluster: Cluster::from(rootdir),


        })




        
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
       fn read_cluster(
           &mut self,
           cluster: Cluster,
           offset: usize,
           buf: &mut [u8]
       ) -> io::Result<usize> {



        let fat_en = fat_entry(cluster.0)?;


        
        match fat_en {
            Status::Data(x) => {
                self.device.read_sector(cluster.0 + offset as u64, buf)
            },
            Status::Eoc(y) => {

            },
            _=> {

            }


        }

        
       }
    



    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
       fn read_chain(
           &mut self,
           start: Cluster,
           buf: &mut Vec<u8>
       ) -> io::Result<usize> {

 


    
       }

    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //     
       fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {

        //find the next cluster 
        ///

        //have the cluster number 

        //get the logical sector specified by the ebpb to physical sectors 
        //virtual to physical
        self.device.read_sector(cluster.0)


        let fat_en = FatEntry(cluster.0);

        Ok(fat_en)


       }
}

impl<'a, HANDLE: VFatHandle> FileSystem for &'a HANDLE {
    type File = crate::traits::Dummy;
    type Dir = crate::traits::Dummy;
    type Entry = crate::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        unimplemented!("FileSystem::open()")
    }
}
