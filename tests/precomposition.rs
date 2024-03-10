// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![recursion_limit = "512"]
use once_cell::sync::Lazy;
use serde_json::{json, Number};
use velato::parser::schema::animation::composition::Composition;
use velato::parser::schema::assets::asset::Asset;
use velato::parser::schema::assets::precomposition::Precomposition;
use velato::parser::schema::helpers::int_boolean::BoolInt;

static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
    json!(
        {
            "id": "precomp_0",
            "fr": 60,
            "nm": "Example",
            "xt": 0,
            "layers": []
        }
    )
});
static PRECOMP: Lazy<Precomposition> = Lazy::new(|| Precomposition {
    asset: Asset {
        id: "precomp_0".to_string(),
        name: Some("Example".to_string()),
    },
    composition: Composition { layers: vec![] },
    frame_rate: Some(Number::from(60)),
    extra: Some(BoolInt::False),
});

#[test]
fn test_serde_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*PRECOMP, actual),
        Err(e) => panic!("{e}"),
    }
}
#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*PRECOMP).unwrap();

    assert_eq!(*JSON, actual)
}
