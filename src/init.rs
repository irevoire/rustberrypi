use clap::{App, Arg};
use env_logger::Env;
use log::info;
use std::collections::HashMap;

pub struct Args {
    pub addr: String,
    pub cookie: String,
}

pub fn init() -> Result<Args, String> {
    kankyo::load().unwrap_or_else(|e| println!("No env file: {}", e));
    env_logger::from_env(Env::default().default_filter_or("rustberrypi")).init();

    let matches = App::new("Rustberrypy 🦀🍇")
        .author(
            "irevoire\t<http://github.com/irevoire>\n\
             Sanzen\t\t<http://github.com/mlemesle>",
        )
        .about("Monitoring raspberry pi")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .env("NAME")
                .default_value("pi") // TODO hostname
                .help("Set the name you'll be identified by")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .env("SERVER")
                .default_value("http://localhost:3000")
                .help("Set the server address to use")
                .takes_value(true),
        )
        .get_matches();
    let name = matches.value_of("name").unwrap();
    let server = matches.value_of("server").unwrap();
    let cookie = get_cookie(server, name)?;

    return Ok(Args {
        cookie: cookie,
        addr: server.to_string(),
    });
}

fn get_cookie(server: &str, name: &str) -> Result<String, String> {
    let mut param = HashMap::new();

    param.insert("name", name);

    info!("send reqwest to get a cookie");

    let client = reqwest::Client::new();
    let res = client.post(&format!("{}/init", server)).json(&param).send();
    let res = match res {
        Ok(r) => r,
        Err(e) => return Err(format!("Could not request a cookie: {}", e)),
    };

    let cookie = match res.headers().get("set-cookie") {
        Some(c) => c,
        None => return Err("There is no cookie in the header!".to_string()),
    };
    let cookie = match cookie.to_str().unwrap().split(";").next() {
        Some(c) => c,
        None => {
            return Err(format!("Bad format in cookie: {:?}", cookie));
        }
    };

    return Ok(cookie.to_string());
}
