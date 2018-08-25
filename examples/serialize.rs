extern crate stash;
extern crate bincode;

use stash::Stash;

#[cfg(feature = "serialization")]
#[macro_use]
extern crate serde;

#[cfg(feature = "serialization")]
#[macro_use]
extern crate serde_derive;

#[derive(Clone,Serialize, Deserialize)]
struct TestItem {
    a: usize,
    b: usize,
    s: String,
}

fn main() -> Result<(), bincode::Error> {
    let mut stash = Stash::new();
    for a in 1..20000 {
        let item= TestItem{a: 0, b: 0, s: String::from("Hello")};
        stash.put(item);
    }
    for a in 1..2000usize {
        stash.take(a);
    }
    let bytes = bincode::serialize(&stash)?;
    println!("Serialized to bincode in {} bytes.", bytes.len());
    Ok(())
}
