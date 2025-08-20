#![allow(clippy::derivable_impls)]

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

use std;

fn main() {
    let content = std::fs::read_to_string( "examples/assets/lottie/google.json").unwrap();
    let v : serde_json::Value = serde_json::from_str( &content ).unwrap();
    let c : Composition = serde_json::from_value( v ).unwrap();
    print!("{:#?}", c);
}