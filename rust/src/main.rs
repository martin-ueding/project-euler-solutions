use project_euler_rust::registry;

fn main() {
    let id: i32 = std::env::args()
        .nth(1)
        .expect("missing id")
        .parse()
        .expect("invalid id");
    println!("Problem: {id}");
    println!("Language: Rust");

    for entry in inventory::iter::<registry::SolutionEntry> {
        if entry.id == id {
            for (name, implementation) in entry.implementations {
                println!("Implementation: {name}");
                let result = implementation();
                println!("Result: {result}");
            }
        }
    }
}
