
extern crate nix;

use nix::unistd::{fork, pipe, close, dup2, execvp, ForkResult};
use nix::sys::wait::{waitpid, WCONTINUED};
use std::ffi::CString;

fn main() {

    let (reader, writer) = pipe().unwrap();
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            close(writer).unwrap();
            dup2(reader, 0);

            let mut argv: Vec<CString> = Vec::new();
            argv.push(CString::new("grep").unwrap());
            argv.push(CString::new("target").unwrap());
            let _result = execvp(&&argv[..][0], &argv[..]);

            waitpid(child, Some(WCONTINUED));
        }
        Ok(ForkResult::Child) => {
            close(reader);
            dup2(writer, 1);
            let mut argv: Vec<CString> = Vec::new();
            argv.push(CString::new("ls").unwrap());
            argv.push(CString::new("-l").unwrap());
            let _result = execvp(&&argv[..][0], &argv[..]);
        }
        Err(_e) => {}
    }

}
