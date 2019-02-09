extern crate apf;

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;

use apf::prelude::*;

use std::env;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    println!(
        r#"

____________    __      ____________,
\_____     /   /_ \     \     _____/
 \_____    \____/__\____/    _____/
  \_____      | APF |       _____/
     \________\__|__/_________/
               /___\
            ._//___\\_.
    "#
    );

    println!(
        "\nAPF server {}\n_______________________________________\n{}\ngit: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_INFO"),
        env!("GIT_REV")
    );

    trace!("starting up...");

    let service = PaymentService::new();

    let public_listening_address = env::var("APF_PUBLIC_LISTENING").unwrap_or("0.0.0.0:8080".to_string());
    let private_listening_address = env::var("APF_PRIVATE_LISTENING").unwrap_or("127.0.0.1:9090".to_string());

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, public_listening_address),
        ApiServer::new(ApiAccess::Private, private_listening_address),
    ]);

    api::start(ApiAggregator::new(vec![service]), config);
}
