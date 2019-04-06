use std::collections::HashMap;

use systemstat::{Platform, System};

pub fn update() {
    let sys = System::new();
    let mut param = HashMap::new();

    if let Ok(uptime) = sys.uptime() {
        param.insert("uptime", uptime.as_secs().to_string());
    }

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/uptime")
        .json(&param)
        .send();

    match res {
        Ok(_) => println!("Reqwest Ok"),
        Err(e) => println!("Reqwest failed : {}", e),
    };
}
