//! Auto-generated lottie parser
//! Raw schema: <https://lottie.github.io/lottie-spec/1.0/lottie.schema.json>

#![allow(clippy::use_self, reason = "Auto-generated code")]
#![allow(clippy::derivable_impls, reason = "Auto-generated code")]

// Include the generated code.
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(test)]
mod tests {
    use crate::Composition;

    #[test]
    fn parse_lottie() {
        let content = std::fs::read_to_string("../examples/assets/lottie/google.json").unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let _c: Composition = serde_json::from_value(v).unwrap();
    }
}
