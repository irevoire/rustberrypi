use log::{error, info, warn};
use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

new_module!(uptime, 1.minutes(), {
    let sys = System::new();
    let mut param = HashMap::new();

    match sys.uptime() {
        Ok(up) => param.insert("uptime", up.as_secs().to_string()),
        Err(e) => {
            return Err(format!("Your system don't allow getting the uptime: {}", e));
        }
    };

    param
});
