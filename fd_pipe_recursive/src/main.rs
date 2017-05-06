
extern crate nix;

use nix::unistd::{fork, pipe, close, dup2, execvp, ForkResult};
use nix::sys::wait::{waitpid, WCONTINUED};
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

            waitpid(child, Some(WCONTINUED));
        }
        Ok(ForkResult::Child) => {
            do_fork(writer0);
        }
        Err(_e) => {}
    }

}

fn do_fork(old_writer: RawFd) {
    let (reader1, writer1) = pipe().unwrap();
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            close(writer1).unwrap();
            dup2(old_writer, 1);
            dup2(reader1, 0);

            let mut argv: Vec<CString> = Vec::new();
            argv.push(CString::new("grep").unwrap());
            argv.push(CString::new("test").unwrap());
            let _result = execvp(&&argv[..][0], &argv[..]);

            waitpid(child, Some(WCONTINUED));
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
