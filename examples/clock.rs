extern crate allan;
extern crate clocksource;

use allan::{Allan, Style};
use clocksource::{Clock, Clocksource};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Calculating stability of time estimators... runtime ~15 minutes");

    println!("Baseline:");
    test_clock(Clock::Monotonic, Clock::Monotonic);

    println!("Realtime:");
    test_clock(Clock::Realtime, Clock::Counter);

    println!("Monotonic:");
    test_clock(Clock::Monotonic, Clock::Counter);
}

fn test_clock(reference: Clock, source: Clock) {
    println!("Reference: {:?} Source: {:?}", reference, source);
    let clock = Clocksource::configured(reference, source);

    let mut allan = Allan::configure()
        .max_tau(100_000)
        .style(Style::Decade)
        .build()
        .unwrap();

    for _ in 0..300_000 {
        let time = clock.time();
        let ref_time = clock.reference();

        let delta = (time as f64 - ref_time as f64) / 1_000_000_000.0;
        allan.record(delta);

        thread::sleep(Duration::new(0, 1_000_000));
    }

    println!("Stability:");
    if let Some(t) = allan.get(1) {
        if let Some(adev) = t.deviation() {
            println!("     ADEV(1mS): {:.3e}", adev);
        }
    }
    if let Some(t) = allan.get(10) {
        if let Some(adev) = t.deviation() {
            println!("    ADEV(10mS): {:.3e}", adev);
        }
    }
    if let Some(t) = allan.get(100) {
        if let Some(adev) = t.deviation() {
            println!("   ADEV(100mS): {:.3e}", adev);
        }
    }
    if let Some(t) = allan.get(1_000) {
        if let Some(adev) = t.deviation() {
            println!("      ADEV(1S): {:.3e}", adev);
        }
    }
    if let Some(t) = allan.get(10_000) {
        if let Some(adev) = t.deviation() {
            println!("     ADEV(10S): {:.3e}", adev);
        }
    }
    if let Some(t) = allan.get(100_000) {
        if let Some(adev) = t.deviation() {
            println!("    ADEV(100S): {:.3e}", adev);
        }
    }
}
