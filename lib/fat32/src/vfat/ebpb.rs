use core::fmt;
use shim::const_assert_size;

use crate::traits::BlockDevice;
use crate::vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    jmp_short_xx_nop: [u8; 3],
    oem_identifier: [u8; 8],
    bytes_per_sector: u16,
    sector_per_cluster: u8,
    reserved_sector: u16,
    number_fat: u8,
    directory_entries: [u8;2],
    total_logical_sectors: [u8;2],
    fat_id : u8,
    sector_per_FAT: [u8;2],
    sector_per_track: [u8;2],
    heads_str_media: [u8;2],
    hidden_sectors: [u8;4],
    logical_sectors: [u8;4],


    size_FAT_sectors: u32,
    flags: [u8;2],
    version_number: [u8;2],
    root_cluster: u32,
    fsinfo_struct_sec: [u8;2],
    backup_boot_sec: : [u8;2],
    reserved_vol: [u8;12],
    drive_number: u8,
    windows_nt: u8,
    signature: u8,
    volume_id: [u8;4],
    volume_label: [u8;11],
    system_identifier: [u8;8],
    boot_code: [u8;420],
    partition_signature: [u8;2],
}

const_assert_size!(BiosParameterBlock, 512);

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        
    	let mut buf = [0u8; 512];

    	let value = device.read(sector, &mut buf)?;

    	if value != 512 {
    		return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")))

    	}

    	let bpb: BiosParameterBlock = unsafe {mem:transmute(buf)};

    	if bpb.partition_signature != 0xAA55 {
    		return Err(Error::BadSignature);
    	}

    	Ok(bpb)

    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
        	.field("partition_signature", &self.partition_signature)


    }
}
