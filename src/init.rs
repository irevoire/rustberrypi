use clap::{App, Arg};
use env_logger::Env;
use log::info;
use std::collections::HashMap;

pub struct Args {
    pub addr: String,
    pub cookie: String,
}

pub fn init() -> Args {
    kankyo::load().unwrap_or_else(|e| println!("No env file: {}", e));
    env_logger::from_env(Env::default().default_filter_or("rustberrypi")).init();

    let matches = App::new("rustberrypy")
        .author("irevoire <irevoire.ovh>")
        .about("Monitor raspberry pi")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .env("NAME")
                .default_value("pi") // TODO hostname
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .env("SERVER")
                .default_value("localhost:3000")
                .takes_value(true),
        )
        .get_matches();
    let name = matches.value_of("name").unwrap();
    let server = matches.value_of("server").unwrap();

    let mut param = HashMap::new();

    param.insert("name", name);

    info!("send reqwest to get a cookie");

    let client = reqwest::Client::new();
    let res = client
        .post("http://192.168.0.21:3000/init")
        .json(&param)
        .send()
        .unwrap();

    let cookie = res.headers().get("set-cookie").unwrap();
    let cookie = cookie.to_str().unwrap().split(";").next().unwrap();

    return Args {
        cookie: cookie.to_string(),
        addr: server.to_string(),
    };
}
