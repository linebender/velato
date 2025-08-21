//! Auto-generated lottie parser build script
//! Raw schema: <https://lottie.github.io/lottie-spec/1.0/lottie.schema.json>

use std::fs;
use std::env;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let response = reqwest::blocking::get("https://lottie.github.io/lottie-spec/1.0/lottie.schema.json")
    .expect("request failed");
    let content = response.text().expect("body invalid");
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");
    fs::write(out_file, contents).unwrap();
}