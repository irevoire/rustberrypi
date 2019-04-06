use std::collections::HashMap;

pub fn init(name: &str) -> String {
    let mut param = HashMap::new();

    param.insert("name", name);

    println!("Sending reqwest");

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/init")
        .json(&param)
        .send()
        .unwrap();

    let cookie = res.headers().get("set-cookie").unwrap();
    let cookie = cookie.to_str().unwrap().split(";").next().unwrap();

    return cookie.to_string();
}
