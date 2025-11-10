// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The schema for a Lottie `Animation`.
//!
//! Raw schema: <https://lottiefiles.github.io/lottie-docs/schema/lottie.schema.json>

pub mod animated_properties;
pub mod animation;
pub mod assets;
pub mod constants;
// todo effect-values
// todo effects
pub mod helpers;
pub mod layers;
pub mod shapes;
pub mod styles;
//todo text

pub use animation::animation::Animation;

#[cfg(test)]
mod tests {
    use crate::schema::{
        animation::composition::Composition, helpers::visual_object::VisualObject,
    };

    use super::{Animation, helpers::int_boolean::BoolInt};
    use once_cell::sync::Lazy;
    use serde_json::json;

    static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
        json!(
            {
                "v": "5.5.2",
                "nm": "Example",
                "fr": 60,
                "ip": 0,
                "op": 60,
                "w": 512,
                "h": 512,
                "ddd": 0,
                "layers": []
            }
        )
    });

    static LOTTIE: Lazy<Animation> = Lazy::new(|| Animation {
        ver: None,
        visual_object: VisualObject {
            name: Some("Example".to_string()),
            ..Default::default()
        },
        composition: Composition { layers: vec![] },
        version: Some("5.5.2".to_string()),
        frame_rate: 60.0,
        in_point: 0.0,
        out_point: 60.0,
        width: 512,
        height: 512,
        three_dimensional: Some(BoolInt::False),
        assets: None,
    });

    #[test]
    fn test_serde_deserialize() {
        let actual = serde_json::from_value(JSON.to_owned());

        match actual {
            Ok(actual) => assert_eq!(*LOTTIE, actual),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_deserialize() {
        let actual = Animation::from_json(JSON.to_owned());

        match actual {
            Ok(actual) => assert_eq!(*LOTTIE, actual),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_can_serialize() {
        serde_json::to_value(&*LOTTIE).unwrap();
    }
}
