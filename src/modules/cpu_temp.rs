use log::{error, info, warn};
use std::collections::HashMap;

use clokwerk::{Scheduler, TimeUnits};
use systemstat::{Platform, System};

new_module!(cpu_temp, 5.seconds(), {
    let sys = System::new();
    let mut param = HashMap::new();

    match sys.cpu_temp() {
        Ok(cput) => param.insert("cpu_temp", cput.to_string()),
        Err(e) => {
            return Err(format!(
                "Your system don't allow getting the cpu temperature: {}",
                e
            ));
        }
    };

    param
});
