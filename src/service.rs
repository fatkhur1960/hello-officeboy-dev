//! Extendable Service interface

use crate::api::ServiceApiBuilder;

///! Base service interface
pub trait Service {
    /// Returns service name
    /// service name must unique between each other.
    fn name(&self) -> &'static str;

    /// Method untuk wiring API.
    fn wire_api(&self, builder: &mut ServiceApiBuilder);
}
