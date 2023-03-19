//! # Rust Basic Macro
//!
//! `RustBasic` is a planned development that aims to make Rust easy to learn, teach, and use.

// rustbasic macro - lib.rs
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(dead_code)]

// #[macro_use]
// mod utils;

/// # How to use
/// 
/// ## 1.
/// ```ignore
/// stopwatch_start!();
/// sleeps(1);
/// stopwatch_stop!();
/// ```
/// ## 2.
/// ```ignore
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.start();
/// sleeps(1);
/// stopwatch.stop();
/// ```

// rust basic macro - macros.rs
use std::time::Instant;
use std::time::Duration;
use std::sync::Mutex;
// use crate::utils::*;

lazy_static::lazy_static! {
    static ref STOPWATCH: Mutex<Option<STOPWATCH>> = Mutex::new(None);
}

struct Stopwatch {
    start_time: Instant,
    stop_time: Instant,
}

impl Stopwatch {

    pub fn new() -> Stopwatch {
        println!("Stopwatch Start...");
        Stopwatch {
            start_time: Instant::now(),
            stop_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.stop_time = self.start_time;
    }

    pub fn stop(&mut self) {
        self.stop_time = Instant::now();
        let elapsed_time = self.duration();
        println!("Stopwatch Stop... Elapsed time: {:?}", elapsed_time);
    }

    pub fn duration(&self) -> Duration {
        self.stop_time - self.start_time
    }
}

/*
#[macro_export]
macro_rules! stopwatch_start {
    () => {
        let stopwatch = Stopwatch::new();
        *STOPWATCH.lock().unwrap() = Some(stopwatch);
    };
}

#[macro_export]
macro_rules! stopwatch_stop {
    () => {
        let mut stopwatch_option = STOPWATCH.lock().unwrap();
        if let Some(ref mut stopwatch) = *stopwatch_option {
            stopwatch.stop();
        }
        *stopwatch_option = None;
    };
}
*/

#[test]
fn test_stopwatch() {
    use std::thread;

    let sleeps = |second| {
        let timeout = Duration::from_secs(second);
        thread::park_timeout(timeout);
    };

//    stopwatch_start!();
//    sleeps(1);
//    stopwatch_stop!();

    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    sleeps(1);
    stopwatch.stop();
}
