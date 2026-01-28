//! A2A (Agent-to-Agent) Protocol Implementation in Rust
//! 
//! This crate provides a Rust implementation of the A2A protocol,
//! which enables communication between AI agents and clients.
//! 
//! The implementation follows the specification defined in the A2A project
//! and provides equivalent functionality to the Python version.

pub mod a2a;

// Re-export the main module for convenience
pub use a2a::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
