//! Solution registration mechanism.
//!
//! We don't want to import the solutions explicitly, rather we want to collect the solutions from their individual files. This means that adding a new solution won't add much code outside of the actual solution file and the `solutions/mod.rs` that we cannot get around of.
pub struct SolutionEntry {
    pub id: i32,
    pub implementations: &'static [(&'static str, fn() -> i64)],
    pub solution: Option<i64>,
}

inventory::collect!(SolutionEntry);
