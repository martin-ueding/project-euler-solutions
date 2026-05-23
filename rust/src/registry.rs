pub struct SolutionEntry {
    pub id: i32,
    pub name: Option<&'static str>,
    pub solve: fn() -> i64,
    pub solution: Option<i64>,
}

inventory::collect!(SolutionEntry);
