extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate slog_atomic;
extern crate slog_async;

use slog::*;
use slog_atomic::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;

use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::time::Duration;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        n => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn main() {
    let decorator = slog_term::PlainDecorator::new(std::io::stdout());
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let drain = AtomicSwitch::new(drain);
    let ctrl = drain.ctrl();
    let root = Logger::root(
        drain.fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"), "build_id" => "8dfljdf")
    );

    let log = root.new(o!("child" => 1));
    let counter = Arc::new(AtomicUsize::new(0));
    let log = log.new(o!("counter" => {
        let counter = counter.clone();
        slog::FnValue(move |_ : &Record| { counter.load(SeqCst) } )
    }));

    // Loggers can be cloned, passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {
            info!(log, "before-fetch-add"); // counter == 0
            counter.fetch_add(1, SeqCst);
            info!(log, "after-fetch-add"); // counter == 1

            let drain = Mutex::new(slog_json::Json::default(std::io::stderr()));

            // AtomicSwitch drain can swap it's interior atomically (race-free)
            ctrl.set(slog::LevelFilter::new(drain, Level::Info).map(slog::Fuse));

            debug!(log, "debug"; "lazy-closure" => FnValue(|_ : &Record| slow_fib(40)));

            info!(log, "subthread"; "stage" => "start");
            thread::sleep(Duration::new(1, 0));
            info!(log, "subthread"; "stage" => "end");
        }
    });

    join.join().unwrap();
}