use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};

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
/// returns if the `exit` command is called.
pub fn shell(prefix: &str) -> ! {
    unimplemented!();

    // let bell:u8 = 0x7;
    // let backspace: u8    = 0x8;
    // let delete: u8   = 127;
    //
    //
    // // /r /n
    //
    // loop {
    //     let mut storage = [0u8; 512];
    //     let max_length = storage.len();
    //     let stack = StackVec::new(&mut storage);
    //
    //
    //
    //
    //     kprint!("{}", prefix);
    //     loop{
    //
    //             let mut console = CONSOLE.lock();
    //             let input_byte =console.read_byte();
    //
    //
    //             if byte == b'\r' || byte == b'\n' {
    //                 //enter
    //
    //
    //
    //             } else {
    //
    //             }







    //
    //     }
    //
    //
    //
    //
    //
    // }
    //
    //
    // if()


}
