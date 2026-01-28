//! Constants for well-known URIs used throughout the A2A Rust SDK
//! 
//! This module provides constants that match the functionality
//! in a2a-python/src/a2a/utils/constants.py

/// Well-known path for the agent card
pub const AGENT_CARD_WELL_KNOWN_PATH: &str = "/.well-known/agent-card.json";

/// Previous well-known path for the agent card (deprecated)
pub const PREV_AGENT_CARD_WELL_KNOWN_PATH: &str = "/.well-known/agent.json";

/// Path for the extended agent card (authenticated)
pub const EXTENDED_AGENT_CARD_PATH: &str = "/agent/authenticatedExtendedCard";

/// Default RPC URL
pub const DEFAULT_RPC_URL: &str = "/";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(AGENT_CARD_WELL_KNOWN_PATH, "/.well-known/agent-card.json");
        assert_eq!(PREV_AGENT_CARD_WELL_KNOWN_PATH, "/.well-known/agent.json");
        assert_eq!(EXTENDED_AGENT_CARD_PATH, "/agent/authenticatedExtendedCard");
        assert_eq!(DEFAULT_RPC_URL, "/");
    }
}
