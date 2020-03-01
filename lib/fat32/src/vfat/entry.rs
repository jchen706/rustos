use crate::traits;
use crate::vfat::{Dir, File, Metadata, VFatHandle};
use core::fmt;

// You can change this definition if you want
#[derive(Debug)]
pub enum Entry<HANDLE: VFatHandle> {
    File(File<HANDLE>),
    Dir(Dir<HANDLE>),
}

// TODO: Implement any useful helper methods on `Entry`.

impl<HANDLE: VFatHandle> traits::Entry for Entry<HANDLE> {
    // FIXME: Implement `traits::Entry` for `Entry
    type File = File<HANDLE>;
    type Dir = Dir<HANDLE>;
    type Metadata = Metadata;

    fn name(&self) -> &str {


    	match self {
    		File(x) => {

    		}

    		Dir<HANDLE> => {

    		}


    	}


        


    }

    fn metadata(&self) -> &Self::Metadata {
      
    }
    fn as_file(&self) -> Option<&Self::File<HANDLE>> {
        
    }
    fn as_dir(&self) -> Option<&Self::Dir<HANDLE>> {
        
    }
    fn into_file(self) -> Option<Self::File<HANDLE>> {
       
    }
    fn into_dir(self) -> Option<Self::Dir<HANDLE>> {
        
    }`





}
