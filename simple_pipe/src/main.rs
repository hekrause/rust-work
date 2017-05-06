
extern crate nix;

use nix::unistd::{fork, pipe, read, write, execvp, ForkResult};
use nix::sys::wait::waitpid;
//use std::os::unix::io::RawFd;
use std::str;

fn main() {

    const BUFFER_SIZE: usize = 32;

    let (reader, writer) = pipe().unwrap();
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            let msg = "Hallo";
            let _size = write(writer, msg.as_bytes());
            let _ret = waitpid(child, None);
        }
        Ok(ForkResult::Child) => {
            let mut read_buffer = [0u8; BUFFER_SIZE];
            let _size = read(reader, &mut read_buffer);
            let msg_received = str::from_utf8(&read_buffer).unwrap();

            println!("{}", msg_received);
        }
        Err(_e) => {}
    }
}
