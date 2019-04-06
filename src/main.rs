use std::{thread, time::Duration};

use clokwerk::Scheduler;

mod init;
mod modules;

fn main() {
    let mut scheduler = Scheduler::new();

    let cookie = init::init("bloubi");
    println!("Got the cookie: {}", cookie);

    modules::cpu_temp::init(&mut scheduler, &cookie);
    modules::uptime::init(&mut scheduler, &cookie);

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
