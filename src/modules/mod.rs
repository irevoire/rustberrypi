#[macro_export]
macro_rules! new_module {
    ($mod_name:ident, $duration:expr, $code:block) => {
        use $crate::init::Args;
        pub fn init(scheduler: &mut clokwerk::Scheduler, args: &Args) {
            if let Err(err) = update(&args.addr, &args.cookie) {
                log::error!(
                    "{} will not be added to the scheduler: {}",
                    stringify!($mod_name),
                    err
                );
                return;
            }

            let cookie = args.cookie.clone();
            let addr = args.addr.clone();

            scheduler
                .every($duration)
                .run(move || update(&addr, &cookie).unwrap_or(()));
        }

        fn update(addr: &String, cookie: &String) -> Result<(), String> {
            let param = $code;

            let client = reqwest::Client::new();
            let res = client
                .post(&format!("{}/{}", addr, stringify!($mod_name)))
                .header("Cookie", cookie.as_bytes())
                .json(&param)
                .send();

            match res {
                Ok(_) => log::info!("{} updated", stringify!($mod_name)),
                Err(e) => log::warn!("{} failed: {}", stringify!($mod_name), e),
            };
            Ok(())
        }
    };
}

pub mod cpu_temp;
pub mod uptime;
