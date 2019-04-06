use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

pub fn init(scheduler: &mut Scheduler, cookie: &String) {
    let cookie = cookie.clone();

    scheduler.every(1.minutes()).run(move || update(&cookie));
}

fn update(cookie: &String) {
    let sys = System::new();
    let mut param = HashMap::new();

    if let Ok(uptime) = sys.uptime() {
        param.insert("uptime", uptime.as_secs().to_string());
    } else {
        return;
    }

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/uptime")
        .header("Cookie", cookie.as_bytes())
        .json(&param)
        .send();

    match res {
        Ok(_) => println!("Reqwest Ok"),
        Err(e) => println!("Reqwest failed : {}", e),
    };
}
