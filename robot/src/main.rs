// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use consert_robot::prelude::*;
use crossbeam::atomic::AtomicCell;
use rand::distributions::Uniform;
use std::io::stdin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use uom::si::f64::Force;
use uom::si::force::newton;

fn main() {
    let monitor = RosMonitor::new();
    let rtp = monitor.rtp.clone();

    let monitor_running = monitor.running.clone();
    let sensor_running = Arc::new(AtomicCell::new(true));

    let power_force_reduction = Arc::new(AtomicCell::new(true));

    let monitor_thread =
        thread::spawn(move || monitor.run_standalone(Frequency::new::<hertz>(0.5)));
    let sensor_thread = thread::spawn({
        let sensor_running = sensor_running.clone();
        let power_force_reduction = power_force_reduction.clone();
        move || {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            while sensor_running.load() {
                let r_old = rtp.load();
                let mut r_new = r_old.clone();
                r_new.mounted_tool_safe = MountedToolSafe::Known(true);
                r_new.power_and_force_reduction =
                    PowerAndForceReduction::Known(power_force_reduction.load());
                r_new.modes_configured = ModesConfigured::Known(true);
                let fast_mode_active: bool = rng.gen();
                r_new.speed_mode_reduced = SpeedModeReduced::Known(fast_mode_active);
                r_new.speed_mode_fast = SpeedModeFast::Known(!fast_mode_active);
                let ran: f64 = rng.sample(Uniform::new(0.0, 3.0));
                r_new.measured_force = MeasuredForce::Known(Force::new::<newton>(ran));
                rtp.compare_and_swap(r_old, r_new);
                thread::sleep(Duration::from_millis(100));
            }
        }
    });
    let stdin = stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('t') => power_force_reduction.store(!power_force_reduction.load()),
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
