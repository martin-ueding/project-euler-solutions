pub struct SolutionEntry {
    pub id: u32,
    pub solve: fn() -> i32,
}

inventory::collect!(SolutionEntry);
