mod store;
mod context_weaver;

pub use store::VectorMemoryStore;
pub use context_weaver::ContextWeaver;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_tier_display() {
        assert_eq!(MemoryTier::Raw.to_string(), "raw");
        assert_eq!(MemoryTier::Consolidated.to_string(), "consolidated");
        assert_eq!(MemoryTier::Verified.to_string(), "verified");
    }
}
