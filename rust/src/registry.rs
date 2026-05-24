pub struct SolutionEntry {
    pub id: i32,
    pub implementations: &'static [(&'static str, fn() -> i64)],
    pub solution: Option<i64>,
}

inventory::collect!(SolutionEntry);
