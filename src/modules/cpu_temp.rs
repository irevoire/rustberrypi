use std::collections::HashMap;

use clokwerk::TimeUnits;
use systemstat::{Platform, System};

new_module!(cpu, 5.seconds(), {
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
