//! Core implementasi untuk Service Payment

use crate::api;
use crate::prelude::*;

use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct TopupQuery {
    pub amount: Option<f64>,
}

/// Core basis service payment
pub struct PaymentService {}

impl PaymentService {
    #[doc(hidden)]
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    /// Rest API endpoint for topup
    fn topup(state: &AppState, query: TopupQuery) -> api::Result<()> {
        trace!("topup account: {:?}", query);
        Err(api::Error::CustomError("Ada masalah bro".to_string(), 555))?;
        Ok(())
    }
}

impl Service for PaymentService {
    fn name(&self) -> &'static str {
        "payment"
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        builder.public_scope().endpoint_mut("v1/topup", Self::topup);
    }
}
