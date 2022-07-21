// SPDX-FileCopyrightText: 2021 Fraunhofer Institute for Experimental Software Engineering IESE
//
// SPDX-License-Identifier: MIT

use consert_camera::prelude::*;
use crossbeam::atomic::AtomicCell;
use std::io::stdin;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use uom::si::f54::Length;
use uom::si::f64::Velocity;
use uom::si::length::meter;
use uom::si::velocity::meter_per_second;

fn main() {
    let monitor = RosMonitor::new();
    let rtp = monitor.rtp.clone();

    let monitor_running = monitor.running.clone();
    let sensor_running = Arc::new(AtomicCell::new(true));

    let installation_approved = Arc::new(AtomicCell::new(true));
    let distance_to_human = Arc::new(AtomicCell::new(true));
    let approximation_speed = Arc::new(AtomicCell::new(true));

    let monitor_thread =
        thread::spawn(move || monitor.run_standalone(Frequency::new::<hertz>(0.5)));
    let sensor_thread = thread::spawn({
        let sensor_running = sensor_running.clone();
        let installation_approved = installation_approved.clone();
        let distance_to_human = distance_to_human.clone();
        let approximation_speed = approximation_speed.clone();

        move || {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            while sensor_running.load() {
                let r_old = rtp.load();
                let mut r_new = r_old.clone();
                r_new.installation_approved = InstallationApproved::Known(true);

                let ran: f64 = rng.sample(Uniform::new(0.0, 6.0));
                r_new.distance_to_human = DistanceToHuman::Known(Length::new<meter>(ran));
                let ran: f64 = rng.sample(Uniform::new(0.0, 4.0));
                r_new.approximation_speed =
                    ApproximationSpeed::Known(Velocity::new::<meter_per_second>(ran));
                
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
