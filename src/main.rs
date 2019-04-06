use std::{thread, time::Duration};

use env_logger::Env;
use log::info;

use clokwerk::Scheduler;

mod init;
mod modules;

fn main() {
    kankyo::load().unwrap_or_else(|e| println!("No env file: {}", e));
    env_logger::from_env(Env::default().default_filter_or("rustberrypi")).init();

    let mut scheduler = Scheduler::new();

    let cookie = init::init("bloubi");
    info!("Got the cookie: {}", cookie);

    modules::cpu_temp::init(&mut scheduler, &cookie);
    modules::uptime::init(&mut scheduler, &cookie);

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(500));
    }
}
