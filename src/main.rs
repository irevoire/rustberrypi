use std::{thread, time::Duration};

use clokwerk::{Scheduler, TimeUnits};

mod init;
mod modules;

fn main() {
    let mut scheduler = Scheduler::new();

    let cookie = init::init("bloubi");
    println!("Got the cookie: {}", cookie);
    let tmpcookie = cookie.clone();
    scheduler
        .every(5.seconds())
        .run(move || modules::cpu_temp::update(&tmpcookie));

    let tmpcookie = cookie.clone();
    scheduler
        .every(1.minutes())
        .run(move || modules::uptime::update(&tmpcookie));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
