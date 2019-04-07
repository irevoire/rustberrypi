use log::{error, info, warn};
use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

pub fn init(scheduler: &mut Scheduler, cookie: &String) {
    if let Err(e) = update(&cookie) {
        error!("cpu_temp will not be added to the scheduler: {}", e);
        return;
    }

    let cookie = cookie.clone();

    scheduler
        .every(5.seconds())
        .run(move || update(&cookie).unwrap_or(()));
}

fn update(cookie: &String) -> Result<(), String> {
    let sys = System::new();
    let mut param = HashMap::new();

    match sys.cpu_temp() {
        Ok(cput) => param.insert("cpu_temp", cput.to_string()),
        Err(e) => {
            return Err(format!(
                "Your system don't allow getting cpu temperature: {}",
                e
            ));
        }
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/cpu")
        .header("Cookie", cookie.as_bytes())
        .json(&param)
        .send();

    match res {
        Ok(_) => info!("cpu_temp updated"),
        Err(e) => warn!("cpu_temp failed: {}", e),
    };
    Ok(())
}
