
extern crate nix;

use nix::unistd::{fork, pipe, close, dup2, execvp, ForkResult};
use nix::sys::wait::{waitpid, WEXITED};
use std::os::unix::io::RawFd;
use std::ffi::CString;

fn main() {

    let (reader0, writer0) = pipe().unwrap();
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            close(writer0).unwrap();
            dup2(reader0, 0);

            let mut argv: Vec<CString> = Vec::new();
            argv.push(CString::new("wc").unwrap());
            argv.push(CString::new("-l").unwrap());
            let _result = execvp(&&argv[..][0], &argv[..]);

            waitpid(child, Some(WEXITED));
        }
        Ok(ForkResult::Child) => {
            let (reader1, writer1) = pipe().unwrap();
            match fork() {
                Ok(ForkResult::Parent { child }) => {
                    close(writer1).unwrap();
                    dup2(writer0, 1);
                    dup2(reader1, 0);

                    let mut argv: Vec<CString> = Vec::new();
                    argv.push(CString::new("grep").unwrap());
                    argv.push(CString::new("test").unwrap());
                    let _result = execvp(&&argv[..][0], &argv[..]);

                    waitpid(child, Some(WEXITED));
                }
                Ok(ForkResult::Child) => {
                    close(reader1);
                    dup2(writer1, 1);
                    let mut argv: Vec<CString> = Vec::new();
                    argv.push(CString::new("cat").unwrap());
                    argv.push(CString::new("test.txt").unwrap());
                    let _result = execvp(&&argv[..][0], &argv[..]);
                }
                Err(_e) => {}
            }
        }
        Err(_e) => {}
    }

}
