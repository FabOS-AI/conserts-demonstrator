// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use consert_workspace::prelude::*;
use crossbeam::atomic::AtomicCell;
use std::io::stdin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

fn main() {
    let monitor = RosMonitor::new();
    let rtp = monitor.rtp.clone();

    let monitor_running = monitor.running.clone();
    let sensor_running = Arc::new(AtomicCell::new(true));

    let monitor_thread =
        thread::spawn(move || monitor.run_standalone(Frequency::new::<hertz>(0.5)));
    let sensor_thread = thread::spawn({
        let sensor_running = sensor_running.clone();
        move || {
            while sensor_running.load() {
                let r_old = rtp.load();
                let mut r_new = r_old.clone();
                r_new.communication_delay =
                    CommunicationDelay::Known(Time::new::<millisecond>(5.0));
                r_new.installation_approved = InstallationApproved::Known(true);
                rtp.compare_and_swap(r_old, r_new);
                thread::sleep(Duration::from_millis(100));
            }
        }
    });
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
    sensor_running.store(false);
    monitor_thread.join().unwrap();
    sensor_thread.join().unwrap();
}
