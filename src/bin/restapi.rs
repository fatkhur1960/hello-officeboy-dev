extern crate apf;

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;
extern crate sodiumoxide;

use apf::prelude::*;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    sodiumoxide::init().expect("Cannot initialize NaCl");

    println!(
        "\nAPF server {}\n\n\
        {}\n\
        git: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_INFO"),
        env!("GIT_REV")
    );

    trace!("starting up...");

    let service = PaymentService::new();

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, "127.0.0.1:8081".to_string()),
        ApiServer::new(ApiAccess::Private, "127.0.0.1:8082".to_string()),
    ]);

    api::start(ApiAggregator::new(vec![service]), config);
}
