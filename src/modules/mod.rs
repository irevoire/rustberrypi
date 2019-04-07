#[macro_export]
macro_rules! new_module {
    ($mod_name:ident, $duration:expr, $code:block) => {
        pub fn init(scheduler: &mut Scheduler, cookie: &String) {
            if let Err(err) = update(&cookie) {
                error!(
                    "{} will not be added to the scheduler: {}",
                    stringify!($mod_name),
                    err
                );
                return;
            }

            let cookie = cookie.clone();

            scheduler
                .every($duration)
                .run(move || update(&cookie).unwrap_or(()));
        }

        fn update(cookie: &String) -> Result<(), String> {
            let param = $code;

            let client = reqwest::Client::new();
            let res = client
                .post(&format!(
                    "http://192.168.0.21:3000/{}",
                    stringify!($mod_name)
                ))
                .header("Cookie", cookie.as_bytes())
                .json(&param)
                .send();

            match res {
                Ok(_) => info!("{} updated", stringify!($mod_name)),
                Err(e) => warn!("{} failed: {}", stringify!($mod_name), e),
            };
            Ok(())
        }
    };
}

pub mod cpu_temp;
pub mod uptime;
