extern crate payment;

#[macro_use]
extern crate log;
extern crate env_logger;

use payment::prelude::*;

fn main() {
    env_logger::init();

    trace!("starting up...");

    let service = PaymentService::new();

    api::start(ApiAggregator::new(vec![service]));
}
