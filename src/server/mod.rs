pub mod java;
pub mod memory;
pub mod server;

pub use java::check_java;
pub use memory::{calculate_max_ram, get_total_ram_gb};
pub use server::run_server;

