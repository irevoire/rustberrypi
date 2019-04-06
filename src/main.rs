use clokwerk::Scheduler;
use std::{thread, time::Duration};

mod init;
mod modules;

fn main() {
    let args = init::init();

    let mut scheduler = Scheduler::new();

    modules::cpu_temp::init(&mut scheduler, &args.cookie);
    modules::uptime::init(&mut scheduler, &args.cookie);

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
