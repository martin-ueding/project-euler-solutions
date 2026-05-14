pub struct SolutionEntry {
    pub id: i32,
    pub solve: fn() -> i64,
}

inventory::collect!(SolutionEntry);
