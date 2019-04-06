use std::{thread, time::Duration};

use clokwerk::{Scheduler, TimeUnits};

mod modules;

fn main() {
    let mut scheduler = Scheduler::new();

    scheduler
        .every(5.seconds())
        .run(|| modules::cpu_temp::update());
    scheduler
        .every(1.minutes())
        .run(|| modules::uptime::update());

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
