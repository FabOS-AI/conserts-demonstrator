// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use consert_scanner::prelude::*;
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

    let s1_unoccupied = Arc::new(AtomicCell::new(true));
    let single_evaluation = Arc::new(AtomicCell::new(true));

    let monitor_thread =
        thread::spawn(move || monitor.run_standalone(Frequency::new::<hertz>(0.5)));
    let sensor_thread = thread::spawn({
        let sensor_running = sensor_running.clone();
        let single_evaluation = single_evaluation.clone();
        let s1_unoccupied = s1_unoccupied.clone();
        move || {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            while sensor_running.load() {
                let r_old = rtp.load();
                let mut r_new = r_old.clone();
                if single_evaluation.load() {
                    r_new.single_evaluation = SingleEvaluation::Known(true);
                } else {
                    r_new.multi_evaluation_4 = MultiEvaluation4::Known(true);
                }
                r_new.installation_approved = InstallationApproved::Known(true);
                r_new.s1_unoccupied = S1Unoccupied::Known(s1_unoccupied.load());
                let s2_unoccupied = s1_unoccupied.load() && rng.gen();
                r_new.s2_unoccupied = S2Unoccupied::Known(s2_unoccupied);
                rtp.compare_and_swap(r_old, r_new);
                thread::sleep(Duration::from_millis(100));
            }
        }
    });
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('m') => single_evaluation.store(!single_evaluation.load()),
            Key::Char('t') => s1_unoccupied.store(!s1_unoccupied.load()),
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
