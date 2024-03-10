use crate::parser::schema::animated_properties::animated_property::{
    AnimatedProperty, AnimatedPropertyK,
};
use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::parser::schema::animated_properties::position::Position;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::parser::schema::{self};
use once_cell::sync::Lazy;

pub static FLOAT_VALUE_ZERO: Lazy<FloatValue> = Lazy::new(|| FloatValue {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(serde_json::Number::from(0)),
    },
});

pub static FLOAT_VALUE_ONE_HUNDRED: Lazy<FloatValue> = Lazy::new(|| FloatValue {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(serde_json::Number::from(100)),
    },
});

pub static MULTIDIM_ONE: Lazy<MultiDimensional> = Lazy::new(|| MultiDimensional {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(vec![
            serde_json::Number::from(1),
            serde_json::Number::from(1),
            serde_json::Number::from(1),
        ]),
    },
    length: None,
});

pub static POSITION_ZERO: Lazy<Position> = Lazy::new(|| Position {
    property_index: None,
    animated: Some(BoolInt::False),
    expression: None,
    length: None,
    value: schema::animated_properties::position::PositionValueK::Static(vec![
        serde_json::Number::from(0),
        serde_json::Number::from(0),
    ]),
});
