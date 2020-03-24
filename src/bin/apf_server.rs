// extern crate apf;

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate env_logger;

use apf::prelude::*;
use apf::service::AuthService;

use std::env;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

//     println!(
//         r#"

// ____________    __      ____________,
// \_____     /   /_ \     \     _____/
//  \_____    \____/__\____/    _____/
//   \_____      | APF |       _____/
//      \________\__|__/_________/
//                /___\
//             ._//___\\_.
//     "#
//     );

    println!(
        "\nHello Officeboy server {}\n_______________________________________\n{}\ngit: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_INFO"),
        env!("GIT_REV")
    );

    trace!("Starting up the server...");

    let auth_service = AuthService::new();
    let payment_service = PaymentService::new();

    let public_listening_address =
        env::var("APF_PUBLIC_LISTENING").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let private_listening_address =
        env::var("APF_PRIVATE_LISTENING").unwrap_or_else(|_| "127.0.0.1:9090".to_string());

    let config = ServiceApiConfig::new(vec![
        ApiServer::new(ApiAccess::Public, public_listening_address),
        ApiServer::new(ApiAccess::Private, private_listening_address),
    ]);

    api::start(ApiAggregator::new(vec![auth_service, payment_service]), config);
}
