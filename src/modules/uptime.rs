use log::{info, warn};
use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

pub fn init(scheduler: &mut Scheduler, cookie: &String) {
    let cookie = cookie.clone();

    scheduler.every(1.minutes()).run(move || update(&cookie));
    info!("added uptime to the scheduler");
}

fn update(cookie: &String) {
    let sys = System::new();
    let mut param = HashMap::new();

    match sys.uptime() {
        Ok(u) => param.insert("uptime", u.as_secs().to_string()),
        Err(e) => {
            warn!("system doesn't allow to get the uptime: {}", e);
            return;
        }
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/uptime")
        .header("Cookie", cookie.as_bytes())
        .json(&param)
        .send();

    match res {
        Ok(_) => info!("Reqwest Ok"),
        Err(e) => warn!("Reqwest failed : {}", e),
    };
}
