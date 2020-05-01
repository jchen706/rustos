use alloc::boxed::Box;
use core::time::Duration;

use smoltcp::wire::{IpAddress, IpEndpoint};

use crate::console::{kprint, CONSOLE};
use crate::param::USER_IMG_BASE;
use crate::process::State;
use crate::traps::TrapFrame;
use crate::{ETHERNET, SCHEDULER};

use kernel_api::*;
use pi::timer::current_time;
use crate::process::Process;

use crate::console::kprintln;


/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
pub fn sys_sleep(ms: u32, tf: &mut TrapFrame) {
    //unimplemented!("sys_sleep()");

    let start = current_time();
    let release_time = start.as_micros() + Duration::from_micros(ms as u64 * 1000).as_micros();

    //Box<dyn FnMut(&mut Process) + Send> 


   kprintln!("Staring system Sleep {} ", start.as_secs() as u64);
   kprintln!("Staring system Sleep End Time {} ", Duration::from_micros(release_time as u64).as_secs() as u64);

   let boxed_fnmut= Box::new(move |p: &mut Process|-> bool {

   	 

    

         let time1 = current_time();


          if time1.as_micros() >= release_time {
            let elapsed_time = time1.as_millis() - start.as_millis();
                p.context.x[0] =  elapsed_time as u64;
                p.context.x[7] =  OsError::Ok as u64;
                kprintln!("End system Sleep End Time {:?} ", release_time as u64);

                return true

      } else {
            return false

      }


      

   	 



   });


   SCHEDULER.switch(State::Waiting(boxed_fnmut), tf);






    //unimplemented!("sys_sleep()")
}

/// Returns current time.
///
/// This system call does not take parameter.
///
/// In addition to the usual status value, this system call returns two
/// parameter:
///  - current time as seconds
///  - fractional part of the current time, in nanoseconds.
pub fn sys_time(tf: &mut TrapFrame) {

   let current = current_time();
   let seconds = current.as_secs();
   let nano = (current-Duration::from_secs(seconds)).as_nanos() as u64;
   

   //.as_nanos();

    //unimplemented!("sys_time()");

  tf.x[0] = seconds;
  tf.x[1] = nano;
  tf.x[7] = OsError::Ok as u64;
     
}

/// Kills the current process.
///
/// This system call does not take paramer and does not return any value.
pub fn sys_exit(tf: &mut TrapFrame) {
   //unimplemented!("sys_exit()");

 kprintln!("Process: {:?} is switch into dead state", tf.tpidr); 
 let a = SCHEDULER.switch(State::Dead, tf);
   
}

/// Writes to console.
///
/// This system call takes one parameter: a u8 character to print.
///
/// It only returns the usual status value.
pub fn sys_write(b: u8, tf: &mut TrapFrame) {

    //unimplemented!("sys_write()");
    
 
      //kprintln!("System writing {}", b);
      if b >= 0 && b <=127 {
        let string = b as char;
        kprint!("{:?}", string);
        tf.x[7] = OsError::Ok as u64;

      } else {
        //kprintln!("System writing Error {}", b as char);
        tf.x[7] = OsError::IoErrorInvalidInput as u64;
      }
      


}

/// Returns the current process's ID.
///
/// This system call does not take parameter.
///
/// In addition to the usual status value, this system call returns a
/// parameter: the current process's ID.
pub fn sys_getpid(tf: &mut TrapFrame) {

    //unimplemented!("sys_getpid()");
    //let id = tf.tpidr;
    tf.x[0]= tf.tpidr;
    tf.x[7] = OsError::Ok as u64;
    
}



/// Creates a socket and saves the socket handle in the current process's
/// socket list.
///
/// This function does neither take any parameter nor return anything,
/// except the usual return code that indicates successful syscall execution.
pub fn sys_sock_create(tf: &mut TrapFrame) {
    // Lab 5 2.D
    unimplemented!("sys_sock_create")
}

/// Returns the status of a socket.
///
/// This system call takes a socket descriptor as the first parameter.
///
/// In addition to the usual status value, this system call returns four boolean
/// values that describes the status of the queried socket.
///
/// - x0: is_active
/// - x1: is_listening
/// - x2: can_send
/// - x3: can_recv
///
/// # Errors
/// This function returns `OsError::InvalidSocket` if a socket that corresponds
/// to the provided descriptor is not found.
pub fn sys_sock_status(sock_idx: usize, tf: &mut TrapFrame) {
    // Lab 5 2.D
    unimplemented!("sys_sock_status")
}

/// Connects a local ephemeral port to a remote IP endpoint with a socket.
///
/// This system call takes a socket descriptor as the first parameter, the IP
/// of the remote endpoint as the second paramter in big endian, and the port
/// number of the remote endpoint as the third parameter.
///
/// `handle_syscall` should read the value of registers and create a struct that
/// implements `Into<IpEndpoint>` when calling this function.
///
/// It only returns the usual status value.
///
/// # Errors
/// This function can return following errors:
///
/// - `OsError::NoEntry`: Fails to allocate an ephemeral port
/// - `OsError::InvalidSocket`: Cannot find a socket that corresponds to the provided descriptor.
/// - `OsError::IllegalSocketOperation`: `connect()` returned `smoltcp::Error::Illegal`.
/// - `OsError::BadAddress`: `connect()` returned `smoltcp::Error::Unaddressable`.
/// - `OsError::Unknown`: All the other errors from calling `connect()`.
pub fn sys_sock_connect(
    sock_idx: usize,
    remote_endpoint: impl Into<IpEndpoint>,
    tf: &mut TrapFrame,
) {
    // Lab 5 2.D
    unimplemented!("sys_sock_connect")
}

/// Listens on a local port for an inbound connection.
///
/// This system call takes a socket descriptor as the first parameter and the
/// local ports to listen on as the second parameter.
///
/// It only returns the usual status value.
///
/// # Errors
/// This function can return following errors:
///
/// - `OsError::InvalidSocket`: Cannot find a socket that corresponds to the provided descriptor.
/// - `OsError::IllegalSocketOperation`: `listen()` returned `smoltcp::Error::Illegal`.
/// - `OsError::BadAddress`: `listen()` returned `smoltcp::Error::Unaddressable`.
/// - `OsError::Unknown`: All the other errors from calling `listen()`.
pub fn sys_sock_listen(sock_idx: usize, local_port: u16, tf: &mut TrapFrame) {
    // Lab 5 2.D
    unimplemented!("sys_sock_listen")
}

/// Returns a slice from a virtual address and a legnth.
///
/// # Errors
/// This functions returns `Err(OsError::BadAddress)` if the slice is not entirely
/// in userspace.
unsafe fn to_user_slice<'a>(va: usize, len: usize) -> OsResult<&'a [u8]> {
    let overflow = va.checked_add(len).is_none();
    if va >= USER_IMG_BASE && !overflow {
        Ok(core::slice::from_raw_parts(va as *const u8, len))
    } else {
        Err(OsError::BadAddress)
    }
}
/// Returns a mutable slice from a virtual address and a legnth.
///
/// # Errors
/// This functions returns `Err(OsError::BadAddress)` if the slice is not entirely
/// in userspace.
unsafe fn to_user_slice_mut<'a>(va: usize, len: usize) -> OsResult<&'a mut [u8]> {
    let overflow = va.checked_add(len).is_none();
    if va >= USER_IMG_BASE && !overflow {
        Ok(core::slice::from_raw_parts_mut(va as *mut u8, len))
    } else {
        Err(OsError::BadAddress)
    }
}

/// Sends data with a connected socket.
///
/// This system call takes a socket descriptor as the first parameter, the
/// address of the buffer as the second parameter, and the length of the buffer
/// as the third parameter.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the number of bytes sent.
///
/// # Errors
/// This function can return following errors:
///
/// - `OsError::InvalidSocket`: Cannot find a socket that corresponds to the provided descriptor.
/// - `OsError::BadAddress`: The address and the length pair does not form a valid userspace slice.
/// - `OsError::IllegalSocketOperation`: `send_slice()` returned `smoltcp::Error::Illegal`.
/// - `OsError::Unknown`: All the other errors from smoltcp.
pub fn sys_sock_send(sock_idx: usize, va: usize, len: usize, tf: &mut TrapFrame) {
    // Lab 5 2.D
    unimplemented!("sys_sock_send")
}

/// Receives data from a connected socket.
///
/// This system call takes a socket descriptor as the first parameter, the
/// address of the buffer as the second parameter, and the length of the buffer
/// as the third parameter.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the number of bytes read.
///
/// # Errors
/// This function can return following errors:
///
/// - `OsError::InvalidSocket`: Cannot find a socket that corresponds to the provided descriptor.
/// - `OsError::BadAddress`: The address and the length pair does not form a valid userspace slice.
/// - `OsError::IllegalSocketOperation`: `recv_slice()` returned `smoltcp::Error::Illegal`.
/// - `OsError::Unknown`: All the other errors from smoltcp.
pub fn sys_sock_recv(sock_idx: usize, va: usize, len: usize, tf: &mut TrapFrame) {
    // Lab 5 2.D
    unimplemented!("sys_sock_recv")
}

/// Writes a UTF-8 string to the console.
///
/// This system call takes the address of the buffer as the first parameter and
/// the length of the buffer as the second parameter.
///
/// In addition to the usual status value, this system call returns the length
/// of the UTF-8 message.
///
/// # Errors
/// This function can return following errors:
///
/// - `OsError::BadAddress`: The address and the length pair does not form a valid userspace slice.
/// - `OsError::InvalidArgument`: The provided buffer is not UTF-8 encoded.
pub fn sys_write_str(va: usize, len: usize, tf: &mut TrapFrame) {
    let result = unsafe { to_user_slice(va, len) }
        .and_then(|slice| core::str::from_utf8(slice).map_err(|_| OsError::InvalidArgument));

    match result {
        Ok(msg) => {
            kprint!("{}", msg);

            tf.x[0] = msg.len() as u64;
            tf.x[7] = OsError::Ok as u64;
        }
        Err(e) => {
            tf.x[7] = e as u64;
        }
    }
}



pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    use crate::console::kprintln;
    //unimplemented!("handle_syscall()")


    if num == 1 {
      kprintln!(" Before calling SLEEP x[0] parameter {:?}", tf.x[0]);
      sys_sleep(tf.x[0] as u32,tf);
    
    } else if num == 2 {
      kprintln!(" Before calling TIMER");

      sys_time(tf);
    } else if num == 3 {
      kprintln!(" Before sys_exit");

      sys_exit(tf);
    }  else if num == 4 {
      //kprintln!(" Before calling writing x[0] parameter {:?}", tf.x[0]);

      sys_write(tf.x[0] as u8 , tf);
    }  else if num == 5 {
      kprintln!("Get PID");

      sys_getpid(tf);
    } else if num == 6 {
      sys_write_str(tf.x[0] as usize, tf.x[1] as usize, tf);
    }

    else {
      tf.x[7] = OsError::Ok as u64;
    }







    //unimplemented!("sys_getpid()")
}





