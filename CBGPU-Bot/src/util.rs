/// Truncates to at most `max` characters (Discord counts characters), adding "…" if cut.
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

/// Rough token estimate that does not over-count multi-byte (Korean) text.
pub fn estimate_tokens(s: &str) -> usize {
    s.chars().count().div_ceil(3)
}

pub const EMBED_DESC_MAX: usize = 4000;
pub const EMBED_FIELD_MAX: usize = 1000;
