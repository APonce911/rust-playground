use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let mut hasher = DefaultHasher::new();
    7920.hash(&mut hasher);
    println!("Hash is {:x}!", hasher.finish());
}
