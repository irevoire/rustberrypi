use std::collections::HashMap;
use systemstat::{Platform, System};

fn main() {
    let mut param = HashMap::new();
    let sys = System::new();

    if let Ok(uptime) = sys.uptime() {
        param.insert("uptime", uptime.as_secs().to_string());
    }

    if let Ok(cpu_temp) = sys.cpu_temp() {
        param.insert("cpu_temp", cpu_temp.to_string());
    }

    let client = reqwest::Client::new();
    let res = client.post("http://192.168.0.21:8081").json(&param).send();
}
