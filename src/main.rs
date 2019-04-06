use std::collections::HashMap;
use std::{thread, time};

use systemstat::{Platform, System};

fn main() {
    let mut param = HashMap::new();
    let sys = System::new();
    let one_sec = time::Duration::from_secs(1);

    loop {
        if let Ok(uptime) = sys.uptime() {
            param.insert("uptime", uptime.as_secs().to_string());
        }

        if let Ok(cpu_temp) = sys.cpu_temp() {
            param.insert("cpu_temp", cpu_temp.to_string());
        }

        let client = reqwest::Client::new();
        let res = client
            .post("http://192.168.0.21:3000/cpu")
            .json(&param)
            .send();
        match res {
            Ok(_) => println!("Reqwest Ok"),
            Err(e) => println!("Reqwest failed : {}", e),
        };

        thread::sleep(one_sec);
    }
}
