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
        let bpb = mbr.get_partition().unwrap()[0];

        if !bpb.is_fat32() {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")));
        }



        //relative offset, offset in sectors from the start of disk to the partition

        let bpb_sector = bpb.get_relative_sector() as u64;

        let ebpb = BiosParameterBlock::from(&mut device, bpb_sector)?;


        let bytes_per_sector = ebpb.get_bytes_per_sector();
        let rootdir = ebpb.get_root_cluster();
        let sect_per_fat = ebpb.get_sector_per_fat();
        let sect_per_cluster = ebpb.get_sector_per_cluster();

        //fat start sector

        //offset of fat from ebpb
        let number_of_reserve_sec = ebpb.get_reserved_sector();
        let fat_start = number_of_reserve_sec as u64;

         


        //data start sector 

        //number of fats * size of fats + fat offset = first address of data region
        //sector per fact *  32 bytes of 16 bytes + fat_sart
        let data_start = ebpb.get_number_FAT() as u64 * sect_per_fat as u64 + fat_start;

        //number of sectors in parition = sectors of fat + sectors of clutster
        let partition1 = Partition {
            start: bpb_sector as u64,
            num_sectors: sect_per_fat as u64 * ebpb.get_number_FAT() as u64 + sect_per_cluster as u64,
            sector_size: bytes_per_sector as u64,

        };


        let cache_partition = CachedPartition::new(device, partition1);


        Ok(VFatHandle::new(VFat{
            phantom: PhantomData<HANDLE>,
            device: cache_partition,
            bytes_per_sector: bytes_per_sector,
            sectors_per_cluster: sect_per_cluster,
            sectors_per_fat: sect_per_fat,
            fat_start_sector: fat_start,
            data_start_sector: data_start as u64,
            rootdir_cluster: Cluster::from(rootdir),

        }))




        
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //  read the bluster 
       fn read_cluster(
           &mut self,
           cluster: Cluster,
           offset: usize,
           buf: &mut [u8]
       ) -> io::Result<usize> {

        //check for the valid of cluster number 
        if cluster.get_clusterValue() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }


        // cluster
        //gets the sector 
        let cluster_start = self.data_start_sector + (cluster.get_clusterValue() as u64 -2) * self.sectors_per_cluster as u64;

        let sector_offset = offset / self.bytes_per_sector as usize;

        let byte_offset = offset % self.bytes_per_sector as usize;



        //get the entry of the of fat32 which contains the next 

        //number of bytes to read from the sector 
        let bytesread:usize = 0;

        //buf len or whole cluster 

        //read all the sectors in the cluster 
        loop {


            //current sector
            let sector = (sector_offset + bytesread) / self.bytes_per_sector as usize; 


            //exit the loop
            if (sector >= self.sectors_per_cluster as usize) {
                break;
            }


            //current byte offset
            byte_offset = (offset + bytesread) % self.bytes_per_sector as usize;

            //read from a sector
            let value:&[u8] = self.device.get(cluster_start + sector_offset as u64)?; 

            //size of return
            let sector_size = value.len() as usize- byte_offset;

            //copy to the buf
            buf[bytesread..bytesread + sector_size].copy_from_slice(&value[byte_offset..]);

            //add the number of bytes read
            bytesread += sector_size;




           
        
        }


        //return bytes copy
        Ok(bytesread as usize) 
   }
    



    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
       fn read_chain(
           &mut self,
           start: Cluster,
           buf: &mut Vec<u8>
       ) -> io::Result<usize> {

        //so we have cluster starting, we read all the sectors from the cluster

        //see the fat entry table -- find the next cluster, repeat the read

        //break on Eoc, or invalid reserve, bad, status


        if start.get_clusterValue() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }


        // cluster
        //gets the sector 
        let cluster_start = self.data_start_sector + (start.get_clusterValue() as u64 -2) * self.sectors_per_cluster as u64;



        let cluster_current = cluster_start;

        let bytesread = 0;

        loop {

            let fat_entry1 = self.fat_entry(cluster_current)?;

            let byte_size = self.read_cluster(cluster_current, 0, &mut buf[bytesread..])?;

            bytesread += byte_size as usize;

            match fat_entry1.status() {
                Status::Data(x) => {
                    cluster_current = x;
                },
                Status::Eoc(y) => {
                    break;
                },
                _ => {

                    //mabye Err
                    break;
                }

            }

        }

        Ok(bytesread)


 


    
       }



        fn read_chain_offset(
           &mut self,
           start: Cluster,
           offset: u64,
           buf: &mut Vec<u8>
       ) -> io::Result<usize> {

        //so we have cluster starting, we read all the sectors from the cluster

        //see the fat entry table -- find the next cluster, repeat the read

        //break on Eoc, or invalid reserve, bad, status


        if start.get_clusterValue() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }


        // cluster
        //gets the sector 
        let cluster_start = self.data_start_sector + (start.get_clusterValue() as u64 -2) * self.sectors_per_cluster as u64;

        let sector_offset = offset / self.bytes_per_sector as usize;



        //offset    //bytes per cluster 
        let cluster_ offset = offset / (self.sector_per_cluster * self.bytes_per_sector)

        //starting cluster with offset 
        cluster_start = cluster_start + cluster_offset;

        //byte offset 
        let byte_offset = offset % self.bytes_per_sector as usize;

        let cluster_current = cluster_start;

        let bytesread = 0;


        //the the offset cluster
        let byte_size = self.read_cluster(cluster_current, byte_offset, &mut buf[bytesread..])?;


        
        bytesread+= byte_size;

        //read 2


        //get 4




        loop {

            let fat_entry1 = self.fat_entry(cluster_current)?; //get 4


            let byte_size = self.read_cluster(cluster_current, 0, &mut buf[bytesread..])?;

            bytesread += byte_size as usize;

            match fat_entry1.status() {
                Status::Data(x) => {
                    cluster_current = x;
                },
                Status::Eoc(y) => {
                    byte_size = self.read_cluster(cluster_current, 0, &mut buf[bytesread..])?;
                    bytesread += byte_size as usize;
                    break;
                },
                _ => {

                    //mabye Err
                    break;
                }

            }

        }

        Ok(bytesread)


 


    
       }

    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //     
       fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {

        //find the cluster 
        //cluster number
        let clusternum = cluster.get_clusterValue();

        //map cluster 

        
        let fat_sector_num, fat_entry_offset = map_cluster_entry(clusternum);

        //have the cluster number 

        //get the logical sector specified by the ebpb to physical sectors 
        //virtual to physical
        //logical sector number 

    
        let value:&[u8] = self.device.get(fat_sector_num)?;

        let f_entry = unsafe{value.cast()}

        Ok(&f_entry[0])


       }


       fn map_cluster_entry(&self, num: u64)-> (u64, u64) {
            let fatsecnum =  self.fat_start_sector + (num * 4) / (self.bytes_per_sector as u64);   
            let fatentryoffset = (num*4) % bytes_per_sector as u64
            (fatsecnum, fatentryoffset)
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
