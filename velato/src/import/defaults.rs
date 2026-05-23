// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::animated_property::{AnimatedProperty, AnimatedPropertyK};
use crate::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::schema::animated_properties::position::Position;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::helpers::int_boolean::BoolInt;
use crate::schema::{self};
use std::sync::LazyLock;

pub static FLOAT_VALUE_ZERO: LazyLock<FloatValue> = LazyLock::new(|| FloatValue {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(0.0),
    },
});

pub static FLOAT_VALUE_ONE_HUNDRED: LazyLock<FloatValue> = LazyLock::new(|| FloatValue {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(100.0),
    },
});

pub static MULTIDIM_ONE: LazyLock<MultiDimensional> = LazyLock::new(|| MultiDimensional {
    animated_property: AnimatedProperty {
        property_index: None,
        animated: Some(BoolInt::False),
        expression: None,
        slot_id: None,
        value: AnimatedPropertyK::Static(vec![1.0, 1.0, 1.0]),
    },
    length: None,
});

pub static POSITION_ZERO: LazyLock<Position> = LazyLock::new(|| Position {
    property_index: None,
    animated: Some(BoolInt::False),
    expression: None,
    length: None,
    value: schema::animated_properties::position::PositionValueK::Static(vec![0.0, 0.0]),
});
