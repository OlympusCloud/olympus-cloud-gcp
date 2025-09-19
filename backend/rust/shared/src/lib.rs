pub mod config;
pub mod database;
pub mod error;
pub mod events;
pub mod models;
pub mod types;

pub use config::AppConfig;
pub use error::{Error, Result, ErrorResponse, ErrorExt};
pub use database::{DatabaseConnection, DatabaseConfig, ConnectionPool, DatabaseContext, DbPool, DbResult};
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_module_compilation() {
        // Basic test to ensure the module compiles
        assert!(true);
    }
}