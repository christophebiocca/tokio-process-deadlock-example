extern crate futures;
extern crate tokio_process;

use std::io;
use std::process::{Command,Stdio};
use tokio_process::CommandExt;
use futures::{Future,IntoFuture,Stream,stream};

// Manually inlining the code at its call site also seems to fix the issue, which seems insane to me?
fn run_command(idx: usize) -> impl Future<Item=bool, Error=io::Error> {
    let child_status = Command::new("ls")
        // The folllowing lines can be uncommented and the bug will still occur.
        //.stdin(Stdio::null())
        //.stdout(Stdio::null())
        //.stderr(Stdio::null())
        .spawn_async();

    child_status.into_future().flatten().map(move |s|{
        println!("Result #{}: {:?}", idx, s);
        s.success()
    })
}

fn main() {
    // Tweaking the total number of tasks doesn't fix the issue.
    // It seems to always hang on the last task.
    let check_stream = stream::iter_ok((0..2).into_iter()).map(|i|{
        run_command(i)
    // Higher levels of buffering still have the bug.
    // Lowering to one fixes the issue (but that's equivalent to not using it).
    // buffer_unordered behaves the same
    }).buffered(2);
    let results = check_stream.collect().wait();
    println!("{:?}", results);
}
