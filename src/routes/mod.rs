//! src/lib.rs
//! src/routes/mod.rs
pub mod health_check;
pub mod subscriptions;
pub use health_check::*;
pub use subscriptions::*;
pub mod email_client;
pub mod subscriptions_confirm;
