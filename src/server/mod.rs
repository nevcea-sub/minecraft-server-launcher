pub mod java;
pub mod memory;
pub mod runner;

pub use java::check_java;
pub use memory::{calculate_max_ram, get_total_ram_gb};
pub use runner::run_server;

