use shim::io;
use shim::path::{Path, PathBuf};

use stack_vec::StackVec;

use pi::atags::Atags;

use fat32::traits::FileSystem;
use fat32::traits::{Dir, Entry};

use crate::console::{kprint, kprintln, CONSOLE};
use crate::ALLOCATOR;
use crate::FILESYSTEM;

use pi::timer::spin_sleep;
use core::time::Duration;


/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns.
pub fn shell(prefix: &str) -> ! {
    //unimplemented!();
    //spin_sleep(Duration::new(5,0));

    let bell:u8 = 0x7;
    let backspace: u8    = 8;
    let delete: u8   = 127;

        

    
    
    
    // /r /n
    //loop through entire script
    loop {
        let mut storage = [0u8; 512];
        let max_length = storage.len();
        let mut stack = StackVec::new(&mut storage);
    
        //let current_length = 0;
        
         // prints > prefix
        kprint!("{} ", prefix);
         
        //let mut console1 = CONSOLE.lock();
        //console.write_byte(b'\n');
        //loop through each line
        loop{
    
                let mut console = CONSOLE.lock();
                let input_byte = console.read_byte();
                //let x = CONSOLE.lock();
                //console.write_byte(input_byte);
                //debug //kprint
                if input_byte == b'\r' || input_byte == b'\n' {
                    //enter
                    let mut str_buffer: [&str; 64] = ["";64];
                    match Command::parse(core::str::from_utf8(stack.into_slice()).unwrap(), &mut str_buffer) {
                        Ok(a) => {
                            if a.path() == "echo" {
                                //kprintln!("{:?}",a.args[1..args.len]);
                                let mut x  = 1;
                                 kprint!("{}","\r\n");
                                for each in a.args {
                                    if x==1{
                                        x=2;
                                        continue;
                                    }
                                    kprint!("{} ", each);
                                }
                                kprint!("{}","\r\n");
                            } else {
                                kprint!("{}","\r\n");
                                kprintln!("unknown command: {}", a.path());

                            }
                            break;
                        },
                        Err(Error::TooManyArgs) => {
                            kprint!("{}","\r\n");
                            kprintln!("{}","error: too many arguments");
                            break;
                        },
                        Err(Error::Empty) => {
                            kprint!("{}","\r\n");
                            break;
                        }


                    }
    
                } else if input_byte == delete || input_byte == backspace {

                    match stack.pop() {
                        None => {
                            kprint!("{}",bell as char);    
                        }
                        Some(a) => {
                            kprint!("{}{}{}", "\u{8}"," ", backspace as char);
                        }
                    };
                    



                } else {
                    

                    if input_byte > 127 {
                            kprint!("{}",bell as char);    
                    } else {
                        
                        match stack.push(input_byte) {
                            Ok(())=> {  

                                kprint!("{}",input_byte as char);    

                            }, 
                            Err(())=> {
                                kprint!("{}",bell as char);    
                                    
                            },
                        }
                    }
                }







    
        }
    
    
    }
    
}
