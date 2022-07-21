// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use consert_binpicking::prelude::*;
use std::io::stdin;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    let monitor = RosMonitor::new();
    let monitor_running = monitor.running.clone();

    let monitor_thread =
        thread::spawn(move || monitor.run_standalone(Frequency::new::<hertz>(0.5)));
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            _ => {
                continue;
            }
        }
    }
    monitor_running.store(false);
    monitor_thread.join().unwrap();
}
