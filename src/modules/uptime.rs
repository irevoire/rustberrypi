use log::{error, info, warn};
use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

pub fn init(scheduler: &mut Scheduler, cookie: &String) {
    if let Err(e) = update(&cookie) {
        error!("uptime will not be added to the scheduler: {}", e);
        return;
    }

    let cookie = cookie.clone();

    scheduler
        .every(1.minutes())
        .run(move || update(&cookie).unwrap_or(()));
    info!("added uptime to the scheduler");
}

fn update(cookie: &String) -> Result<(), String> {
    let sys = System::new();
    let mut param = HashMap::new();

    match sys.uptime() {
        Ok(u) => param.insert("uptime", u.as_secs().to_string()),
        Err(e) => return Err(format!("Your system don't allow uptime: {}", e)),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/uptime")
        .header("Cookie", cookie.as_bytes())
        .json(&param)
        .send();

    match res {
        Ok(_) => info!("uptime updated"),
        Err(e) => warn!("uptime failed : {}", e),
    };
    Ok(())
}
