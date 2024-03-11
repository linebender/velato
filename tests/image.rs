// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use once_cell::sync::Lazy;
use serde_json::{json, Number};
use velato::parser::schema::assets::asset::Asset;
use velato::parser::schema::assets::file_asset::FileAsset;
use velato::parser::schema::assets::image::Image;
use velato::parser::schema::helpers::int_boolean::BoolInt;

static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
    json!(
        {
            "id": "my image",
            "h": 512,
            "w": 512,
            "e": 1,
            "p": "data:image/png;base64,..."
        }
    )
});
static IMAGE: Lazy<Image> = Lazy::new(|| Image {
    file_asset: FileAsset {
        asset: Asset {
            id: "my image".to_string(),
            name: None,
        },
        dir: None,
        file_name: "data:image/png;base64,...".to_string(),
        embedded: Some(BoolInt::True),
    },
    height: Some(Number::from(512)),
    width: Some(Number::from(512)),
    sequence: None,
});

#[test]
fn test_deserialize() {
    let actual = serde_json::from_value(JSON.to_owned());

    match actual {
        Ok(actual) => assert_eq!(*IMAGE, actual),
        Err(e) => panic!("{e}"),
    }
}

#[test]
fn test_serialize() {
    let actual = serde_json::to_value(&*IMAGE).unwrap();

    assert_eq!(*JSON, actual)
}
