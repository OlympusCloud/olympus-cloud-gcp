pub mod database;
pub mod error;
pub mod events;
pub mod security;
pub mod types;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_module_compilation() {
        // Basic test to ensure the module compiles
        assert!(true);
    }
}