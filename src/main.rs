extern crate futures;
extern crate tokio_signal;
extern crate libc;

use std::io;
use std::time::Duration;
use std::thread::{self,JoinHandle};
use std::process::{Command,ExitStatus};
use tokio_signal::unix::Signal;
use futures::{Future,Stream};

// Manually inlining the code at its call site also seems to fix the issue, which seems insane to me?
fn run_command(idx: usize) -> JoinHandle<Result<ExitStatus, io::Error>> {
    thread::spawn(
        move ||{
            let status = Command::new("echo")
                .arg(format!("I am spawned process #{}", idx))
                .status();
            println!("Task {} finished with status {:?}", idx, status);
            status
        },
    )
}

fn main() {
    for i in 0..11 {
        run_command(i);
    }
    let signal = Signal::new(libc::SIGCHLD);
    for notif in signal.flatten_stream().wait() {
        println!("Notified: {:?}", notif);
    }
}
