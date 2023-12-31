use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use hashbrown::HashMap;
use shim::io;

use crate::traits::BlockDevice;

#[derive(Debug)]
struct CacheEntry {
    data: Vec<u8>,
    dirty: bool,
}

pub struct Partition {
    /// The physical sector where the partition begins.
    pub start: u64,
    /// Number of sectors
    pub num_sectors: u64,
    /// The size, in bytes, of a logical sector in the partition.
    pub sector_size: u64,
}

pub struct CachedPartition {
    device: Box<dyn BlockDevice>,
    cache: HashMap<u64, CacheEntry>,
    partition: Partition,
}

impl CachedPartition {

    pub fn sector_size(&self)-> u64 {
        self.partition.sector_size
    }
    /// Creates a new `CachedPartition` that transparently caches sectors from
    /// `device` and maps physical sectors to logical sectors inside of
    /// `partition`. All reads and writes from `CacheDevice` are performed on
    /// in-memory caches.
    ///
    /// The `partition` parameter determines the size of a logical sector and
    /// where logical sectors begin. An access to a sector `0` will be
    /// translated to physical sector `partition.start`. Virtual sectors of
    /// sector number `[0, num_sectors)` are accessible.
    ///
    /// `partition.sector_size` must be an integer multiple of
    /// `device.sector_size()`.
    ///
    /// # Panics
    ///
    /// Panics if the partition's sector size is < the device's sector size.
    pub fn new<T>(device: T, partition: Partition) -> CachedPartition
    where
        T: BlockDevice + 'static,
    {
        assert!(partition.sector_size >= device.sector_size());

        CachedPartition {
            device: Box::new(device),
            cache: HashMap::new(),
            partition: partition,
        }
    }

    /// Returns the number of physical sectors that corresponds to
    /// one logical sector.
    fn factor(&self) -> u64 {
        self.partition.sector_size / self.device.sector_size()
    }

    /// Maps a user's request for a sector `virt` to the physical sector.
    /// Returns `None` if the virtual sector number is out of range.
    fn virtual_to_physical(&self, virt: u64) -> Option<u64> {
        if virt >= self.partition.num_sectors {
            return None;
        }

        let physical_offset = virt * self.factor();
        let physical_sector = self.partition.start + physical_offset;

        Some(physical_sector)
    }

    /// Returns a mutable reference to the cached sector `sector`. If the sector
    /// is not already cached, the sector is first read from the disk.
    ///
    /// The sector is marked dirty as a result of calling this method as it is
    /// presumed that the sector will be written to. If this is not intended,
    /// use `get()` instead.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get_mut(&mut self, sector: u64) -> io::Result<&mut [u8]> {

        if !self.cache.contains_key(&sector) {
            
            let physical_sector = self.virtual_to_physical(sector);

            match physical_sector {
                None => {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not determine physical sector in get mut function"));
                },
                Some(x) => {
                    let physical_section = physical_sector.unwrap();


                     let num_ps = self.factor();
                     let mut buf:Vec<u8> = Vec::new();

                    for i in 0..num_ps {
                        let value = self.device.read_sector(physical_section + i, &mut buf)?;
                    } 

                    self.cache.insert(sector, CacheEntry {data:buf, dirty: false});

                    //read the entire physical sector
                    //need to find the number of physical sector by the logical sector
           
                },
            }
        }
            
            let mut cacheentry = self.cache.get_mut(&sector).unwrap();
            cacheentry.dirty = true;
            let data1 = cacheentry.data.as_mut_slice();

            // convert vec<u8> to [u8]
            Ok(data1)
        
        
        
    }

    /// Returns a reference to the cached sector `sector`. If the sector is not
    /// already cached, the sector is first read from the disk.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get(&mut self, sector: u64) -> io::Result<&[u8]> {
        
        if !self.cache.contains_key(&sector) {
            
            let physical_sector = self.virtual_to_physical(sector);
            if physical_sector == None {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not determine physical sector in get function"));
            }

            let physical_section = physical_sector.unwrap();

            //read the entire physical sector
            //need to find the number of physical sector by the logical sector
            let num_ps = self.factor();
            let mut buf = Vec::new();

            for i in 0..num_ps {
                let value = self.device.read_all_sector(physical_section + i, &mut buf)?;
            } 

            self.cache.insert(sector, CacheEntry {data:buf, dirty: false});

        }

            let mut cacheentry = self.cache.get(&sector).unwrap();
            //cacheentry.dirty = true
            let data1 = &cacheentry.data[..];

            // convert vec<u8> to [u8]
            Ok(&data1)
        
        
    }






}

// FIXME: Implement `BlockDevice` for `CacheDevice`. The `read_sector` and
// `write_sector` methods should only read/write from/to cached sectors.
impl BlockDevice for CachedPartition {
    fn sector_size(&self) -> u64 {
        self.partition.sector_size
    }

    fn read_sector(&mut self, sector: u64, buf: &mut [u8]) -> io::Result<usize> {
        //read sector
        let value: &[u8] = self.get(sector)?;

        //needs to read from cache
        buf.copy_from_slice(&value[..]);
        
        Ok(buf.len())
        

    }

    fn write_sector(&mut self, sector: u64, buf: &[u8]) -> io::Result<usize> {
        let mut value = self.get_mut(sector)?;


        //needs to write into the cache
        value.copy_from_slice(&buf[..]);
        Ok(buf.len())

    }
}

impl fmt::Debug for CachedPartition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CachedPartition")
            .field("device", &"<block device>")
            .field("cache", &self.cache)
            .finish()
    }
}
