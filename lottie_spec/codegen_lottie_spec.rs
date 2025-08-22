#![feature(prelude_import)]
#![allow(clippy::derivable_impls)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`Assets`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "all-assets": {
///    "oneOf": [
///      {
///        "$ref": "#/$defs/assets/precomposition"
///      },
///      {
///        "$ref": "#/$defs/assets/image"
///      }
///    ]
///  },
///  "asset": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/visual-object"
///      },
///      {
///        "properties": {
///          "id": {
///            "description": "Unique identifier used by layers when referencing this asset",
///            "title": "ID",
///            "type": "string"
///          }
///        },
///        "required": [
///          "id"
///        ],
///        "type": "object"
///      }
///    ],
///    "title": "Asset",
///    "type": "object"
///  },
///  "image": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/assets/asset"
///      },
///      {
///        "$ref": "#/$defs/helpers/slottable-object"
///      },
///      {
///        "allOf": [
///          {
///            "if": {
///              "properties": {
///                "e": {
///                  "const": 1
///                }
///              },
///              "required": [
///                "e"
///              ]
///            },
///            "then": {
///              "properties": {
///                "p": {
///                  "$ref": "#/$defs/values/data-url"
///                }
///              }
///            }
///          }
///        ],
///        "else": {
///          "required": [
///            "w",
///            "h",
///            "p"
///          ]
///        },
///        "if": {
///          "required": [
///            "sid"
///          ]
///        },
///        "properties": {
///          "e": {
///            "$ref": "#/$defs/values/int-boolean",
///            "description": "If '1', 'p' is a Data URL",
///            "title": "Embedded"
///          },
///          "h": {
///            "description": "Height of the image",
///            "title": "Height",
///            "type": "number"
///          },
///          "p": {
///            "description": "Name of the image file or a data url",
///            "title": "File Name",
///            "type": "string"
///          },
///          "u": {
///            "description": "Path to the image file",
///            "title": "File Path",
///            "type": "string"
///          },
///          "w": {
///            "description": "Width of the image",
///            "title": "Width",
///            "type": "number"
///          }
///        },
///        "type": "object"
///      }
///    ],
///    "description": "Asset containing an image that can be referenced by layers.",
///    "title": "Image",
///    "type": "object"
///  },
///  "precomposition": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/assets/asset"
///      },
///      {
///        "$ref": "#/$defs/composition/composition"
///      }
///    ],
///    "description": "Asset containing a composition that can be referenced by layers.",
///    "title": "Precomposition",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Assets(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Assets {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Assets { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Assets {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Assets {
    #[inline]
    fn clone(&self) -> Assets {
        Assets(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Assets {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Assets", &&self.0)
    }
}
impl ::std::ops::Deref for Assets {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Assets> for ::serde_json::Value {
    fn from(value: Assets) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Assets> for Assets {
    fn from(value: &Assets) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Assets {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Composition`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "animation": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/visual-object"
///      },
///      {
///        "properties": {
///          "assets": {
///            "description": "List of assets that can be referenced by layers",
///            "items": {
///              "$ref": "#/$defs/assets/all-assets"
///            },
///            "title": "Assets",
///            "type": "array"
///          },
///          "fr": {
///            "description": "Framerate in frames per second",
///            "exclusiveMinimum": 0,
///            "title": "Framerate",
///            "type": "number"
///          },
///          "h": {
///            "description": "Height of the animation",
///            "minimum": 0,
///            "title": "Height",
///            "type": "integer"
///          },
///          "ip": {
///            "description": "Frame the animation starts at (usually 0)",
///            "title": "In Point",
///            "type": "number"
///          },
///          "markers": {
///            "description": "Markers defining named sections of the composition.",
///            "items": {
///              "$ref": "#/$defs/helpers/marker"
///            },
///            "title": "Markers",
///            "type": "array"
///          },
///          "op": {
///            "description": "Frame the animation stops/loops at, which makes this the duration in frames when `ip` is 0",
///            "title": "Out Point",
///            "type": "number"
///          },
///          "slots": {
///            "additionalProperties": {
///              "$ref": "#/$defs/helpers/slot"
///            },
///            "description": "Dictionary of slot ids that will replace matching properties.",
///            "title": "Slots",
///            "type": "object"
///          },
///          "ver": {
///            "description": "Specification version this Lottie is targeting. This is a 6 digit number with version components encoded as `MMmmpp`, with `MM` being major version, `mm` being minor and `pp` being patch.",
///            "minimum": 10000,
///            "title": "Specification Version",
///            "type": "integer"
///          },
///          "w": {
///            "description": "Width of the animation",
///            "minimum": 0,
///            "title": "Width",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "w",
///          "h",
///          "fr",
///          "op",
///          "ip"
///        ],
///        "type": "object"
///      },
///      {
///        "$ref": "#/$defs/composition/composition"
///      }
///    ],
///    "description": "Top level object, describing the animation",
///    "title": "Animation",
///    "type": "object"
///  },
///  "composition": {
///    "description": "An object that contains a list of layers",
///    "properties": {
///      "layers": {
///        "items": {
///          "$ref": "#/$defs/layers/all-layers"
///        },
///        "title": "Layers",
///        "type": "array"
///      }
///    },
///    "required": [
///      "layers"
///    ],
///    "title": "Composition",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Composition(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Composition {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Composition { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Composition {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Composition {
    #[inline]
    fn clone(&self) -> Composition {
        Composition(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Composition {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Composition", &&self.0)
    }
}
impl ::std::ops::Deref for Composition {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Composition> for ::serde_json::Value {
    fn from(value: Composition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Composition> for Composition {
    fn from(value: &Composition) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Composition {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Constants`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "fill-rule": {
///    "description": "Rule used to handle multiple shapes rendered with the same fill object",
///    "oneOf": [
///      {
///        "const": 1,
///        "description": "Everything is colored (You can think of this as an OR)",
///        "title": "Non Zero"
///      },
///      {
///        "const": 2,
///        "description": "Colored based on intersections and path direction, can be used to create \"holes\"",
///        "title": "Even Odd"
///      }
///    ],
///    "title": "Fill Rule",
///    "type": "integer"
///  },
///  "gradient-type": {
///    "description": "Whether a Gradient is a linear or radial.",
///    "oneOf": [
///      {
///        "const": 1,
///        "description": "Colors transition in a single linear direction.",
///        "title": "Linear"
///      },
///      {
///        "const": 2,
///        "description": "Colors transition outward from a center point.",
///        "title": "Radial"
///      }
///    ],
///    "title": "Gradient Type",
///    "type": "integer"
///  },
///  "line-cap": {
///    "description": "Style at the end of a stoked line",
///    "oneOf": [
///      {
///        "const": 1,
///        "title": "Butt"
///      },
///      {
///        "const": 2,
///        "title": "Round"
///      },
///      {
///        "const": 3,
///        "title": "Square"
///      }
///    ],
///    "title": "Line Cap",
///    "type": "integer"
///  },
///  "line-join": {
///    "description": "Style at a sharp corner of a stoked line",
///    "oneOf": [
///      {
///        "const": 1,
///        "title": "Miter"
///      },
///      {
///        "const": 2,
///        "title": "Round"
///      },
///      {
///        "const": 3,
///        "title": "Bevel"
///      }
///    ],
///    "title": "Line Join",
///    "type": "integer"
///  },
///  "mask-mode": {
///    "description": "Describes how a mask interacts (blends) with the preceding masks in the stack.",
///    "oneOf": [
///      {
///        "const": "n",
///        "description": "The mask is ignored.",
///        "title": "None"
///      },
///      {
///        "const": "a",
///        "description": "Mask coverage is added (Normal blending).",
///        "title": "Add"
///      },
///      {
///        "const": "s",
///        "description": "Mask coverage is subtracted (Subtract blending).",
///        "title": "Subtract"
///      },
///      {
///        "const": "i",
///        "description": "Mask coverage is intersected (Source-In blending).",
///        "title": "Intersect"
///      }
///    ],
///    "title": "Mask Mode",
///    "type": "string"
///  },
///  "matte-mode": {
///    "description": "How a layer should mask another layer",
///    "oneOf": [
///      {
///        "const": 0,
///        "description": "The layer is not used as a track matte",
///        "title": "Normal"
///      },
///      {
///        "const": 1,
///        "description": "The masked layer opacity is modulated by the track matte layer opacity",
///        "title": "Alpha"
///      },
///      {
///        "const": 2,
///        "description": "The masked layer opacity is modulated by the inverted track matte layer opacity",
///        "title": "Inverted Alpha"
///      },
///      {
///        "const": 3,
///        "description": "The masked layer opacity is modulated by the track matte layer luminance",
///        "title": "Luma"
///      },
///      {
///        "const": 4,
///        "description": "The masked layer opacity is modulated by the inverted track matte layer luminance",
///        "title": "Inverted Luma"
///      }
///    ],
///    "title": "Matte Mode",
///    "type": "integer"
///  },
///  "shape-direction": {
///    "description": "Drawing direction of the shape curve, useful for trim path",
///    "oneOf": [
///      {
///        "const": 1,
///        "description": "Usually clockwise",
///        "title": "Normal"
///      },
///      {
///        "const": 3,
///        "description": "Usually counter clockwise",
///        "title": "Reversed"
///      }
///    ],
///    "title": "Shape Direction",
///    "type": "integer"
///  },
///  "star-type": {
///    "description": "Whether a PolyStar is a star or a polygon",
///    "oneOf": [
///      {
///        "const": 1,
///        "title": "Star"
///      },
///      {
///        "const": 2,
///        "title": "Polygon"
///      }
///    ],
///    "title": "Star Type",
///    "type": "integer"
///  },
///  "stroke-dash-type": {
///    "description": "Type of a dash item in a stroked line",
///    "oneOf": [
///      {
///        "const": "d",
///        "title": "Dash"
///      },
///      {
///        "const": "g",
///        "title": "Gap"
///      },
///      {
///        "const": "o",
///        "title": "Offset"
///      }
///    ],
///    "title": "Stroke Dash Type",
///    "type": "string"
///  },
///  "trim-multiple-shapes": {
///    "description": "How to handle multiple shapes in trim path",
///    "oneOf": [
///      {
///        "const": 1,
///        "description": "All shapes apply the trim at the same time",
///        "title": "Parallel"
///      },
///      {
///        "const": 2,
///        "description": "Shapes are considered as a continuous sequence",
///        "title": "Sequential"
///      }
///    ],
///    "title": "Trim Multiple Shapes",
///    "type": "integer"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Constants(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Constants {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Constants { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Constants {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Constants {
    #[inline]
    fn clone(&self) -> Constants {
        Constants(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Constants {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Constants", &&self.0)
    }
}
impl ::std::ops::Deref for Constants {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Constants> for ::serde_json::Value {
    fn from(value: Constants) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Constants> for Constants {
    fn from(value: &Constants) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Constants {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Helpers`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "marker": {
///    "description": "Defines named portions of the composition.",
///    "properties": {
///      "cm": {
///        "title": "Comment",
///        "type": "string"
///      },
///      "dr": {
///        "title": "Duration",
///        "type": "number"
///      },
///      "tm": {
///        "title": "Time",
///        "type": "number"
///      }
///    },
///    "title": "Marker",
///    "type": "object"
///  },
///  "mask": {
///    "allOf": [
///      {
///        "properties": {
///          "mode": {
///            "$ref": "#/$defs/constants/mask-mode",
///            "default": "i",
///            "title": "Mode"
///          },
///          "o": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "default": 100,
///            "description": "Mask opacity, as a percentage [0..100].",
///            "title": "Opacity"
///          },
///          "pt": {
///            "$ref": "#/$defs/properties/bezier-property",
///            "description": "Mask shape",
///            "title": "Shape"
///          }
///        },
///        "required": [
///          "pt"
///        ]
///      }
///    ],
///    "description": "Mask for layer content.",
///    "title": "Mask",
///    "type": "object"
///  },
///  "slot": {
///    "description": "Defines a property value that will be set to all matched properties",
///    "properties": {
///      "p": {
///        "description": "Property Value",
///        "title": "Property Value"
///      }
///    },
///    "required": [
///      "p"
///    ],
///    "title": "Slot",
///    "type": "object"
///  },
///  "slottable-object": {
///    "description": "Object that may have its value replaced with a slot value",
///    "properties": {
///      "sid": {
///        "description": "Identifier to look up the slot",
///        "title": "Slot Id",
///        "type": "string"
///      }
///    },
///    "title": "Slottable Object",
///    "type": "object"
///  },
///  "slottable-property": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/slottable-object"
///      }
///    ],
///    "description": "Property that may have its value replaced with a slot value",
///    "else": {
///      "required": [
///        "a",
///        "k"
///      ]
///    },
///    "if": {
///      "required": [
///        "sid"
///      ]
///    },
///    "title": "Slottable Property",
///    "type": "object"
///  },
///  "transform": {
///    "allOf": [
///      {
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/properties/position-property",
///            "description": "Anchor point: a position (relative to its parent) around which transformations are applied (ie: center for rotation / scale)",
///            "title": "Anchor Point"
///          },
///          "o": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "title": "Opacity"
///          },
///          "p": {
///            "$ref": "#/$defs/properties/splittable-position-property",
///            "description": "Position / Translation",
///            "title": "Position"
///          },
///          "r": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Rotation in degrees, clockwise",
///            "title": "Rotation"
///          },
///          "s": {
///            "$ref": "#/$defs/properties/vector-property",
///            "description": "Scale factor, `[100, 100]` for no scaling",
///            "title": "Scale"
///          },
///          "sa": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Direction along which skew is applied, in degrees (`0` skews along the X axis, `90` along the Y axis)",
///            "title": "Skew Axis"
///          },
///          "sk": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Skew amount as an angle in degrees",
///            "title": "Skew"
///          }
///        }
///      }
///    ],
///    "description": "Layer transform",
///    "title": "Transform",
///    "type": "object"
///  },
///  "visual-object": {
///    "allOf": [
///      {
///        "properties": {
///          "nm": {
///            "description": "Human readable name, as seen from editors and the like",
///            "title": "Name",
///            "type": "string"
///          }
///        },
///        "required": [],
///        "type": "object"
///      }
///    ],
///    "description": "",
///    "title": "Visual Object",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Helpers(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Helpers {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Helpers { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Helpers {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Helpers {
    #[inline]
    fn clone(&self) -> Helpers {
        Helpers(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Helpers {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Helpers", &&self.0)
    }
}
impl ::std::ops::Deref for Helpers {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Helpers> for ::serde_json::Value {
    fn from(value: Helpers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Helpers> for Helpers {
    fn from(value: &Helpers) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Helpers {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Layers`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "all-layers": {
///    "oneOf": [
///      {
///        "$ref": "#/$defs/layers/precomposition-layer"
///      },
///      {
///        "$ref": "#/$defs/layers/image-layer"
///      },
///      {
///        "$ref": "#/$defs/layers/null-layer"
///      },
///      {
///        "$ref": "#/$defs/layers/solid-layer"
///      },
///      {
///        "$ref": "#/$defs/layers/shape-layer"
///      },
///      {
///        "$ref": "#/$defs/layers/unknown-layer"
///      }
///    ]
///  },
///  "image-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/visual-layer"
///      },
///      {
///        "properties": {
///          "refId": {
///            "description": "ID of the image as specified in the assets",
///            "title": "Reference Id",
///            "type": "string"
///          },
///          "ty": {
///            "const": 2,
///            "description": "Layer type",
///            "title": "Type",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty",
///          "refId"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Layer containing an image",
///    "title": "Image Layer",
///    "type": "object"
///  },
///  "layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/visual-object"
///      },
///      {
///        "properties": {
///          "hd": {
///            "description": "Whether the layer is hidden",
///            "title": "Hidden",
///            "type": "boolean"
///          },
///          "ind": {
///            "description": "Index that can be used for parenting and referenced in expressions",
///            "title": "Index",
///            "type": "integer"
///          },
///          "ip": {
///            "description": "Frame when the layer becomes visible",
///            "title": "In Point",
///            "type": "number"
///          },
///          "op": {
///            "description": "Frame when the layer becomes invisible",
///            "title": "Out Point",
///            "type": "number"
///          },
///          "parent": {
///            "description": "Must be the `ind` property of another layer",
///            "title": "Parent Index",
///            "type": "integer"
///          },
///          "ty": {
///            "description": "Layer Type",
///            "title": "Type",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty",
///          "ip",
///          "op"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Common properties for all layers",
///    "title": "Layer",
///    "type": "object"
///  },
///  "null-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/visual-layer"
///      },
///      {
///        "properties": {
///          "ty": {
///            "const": 3,
///            "description": "Layer type",
///            "title": "Type",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Layer with no data, useful to group layers together",
///    "title": "Null Layer",
///    "type": "object"
///  },
///  "precomposition-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/visual-layer"
///      },
///      {
///        "properties": {
///          "h": {
///            "description": "Height of the clipping rect",
///            "title": "Height",
///            "type": "integer"
///          },
///          "refId": {
///            "description": "ID of the precomp as specified in the assets",
///            "title": "Reference Id",
///            "type": "string"
///          },
///          "sr": {
///            "default": 1,
///            "title": "Time Stretch",
///            "type": "number"
///          },
///          "st": {
///            "default": 0,
///            "title": "Start Time",
///            "type": "number"
///          },
///          "tm": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Timeline remap function (frame index -> time in seconds)",
///            "title": "Time Remap"
///          },
///          "ty": {
///            "const": 0,
///            "description": "Layer type",
///            "title": "Type",
///            "type": "integer"
///          },
///          "w": {
///            "description": "Width of the clipping rect",
///            "title": "Width",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty",
///          "refId"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Layer that renders a Precomposition asset",
///    "title": "Precomposition Layer",
///    "type": "object"
///  },
///  "shape-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/visual-layer"
///      },
///      {
///        "properties": {
///          "shapes": {
///            "items": {
///              "$ref": "#/$defs/shapes/all-graphic-elements"
///            },
///            "title": "Shapes",
///            "type": "array"
///          },
///          "ty": {
///            "const": 4,
///            "description": "Layer type",
///            "title": "Type",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty",
///          "shapes"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Layer containing Shapes",
///    "title": "Shape Layer",
///    "type": "object"
///  },
///  "solid-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/visual-layer"
///      },
///      {
///        "properties": {
///          "sc": {
///            "$ref": "#/$defs/values/hexcolor",
///            "description": "Solid fill color",
///            "title": "Color"
///          },
///          "sh": {
///            "description": "Solid rectangle height",
///            "title": "Height",
///            "type": "integer"
///          },
///          "sw": {
///            "description": "Solid rectangle width",
///            "title": "Width",
///            "type": "integer"
///          },
///          "ty": {
///            "const": 1,
///            "description": "Layer type",
///            "title": "Type",
///            "type": "integer"
///          }
///        },
///        "required": [
///          "ty",
///          "sw",
///          "sh",
///          "sc"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Solid color, rectangle-shaped layer",
///    "title": "Solid Layer",
///    "type": "object"
///  },
///  "unknown-layer": {
///    "description": "Unknown layer types. Types not defined by the specification are still allowed.",
///    "properties": {
///      "ty": {
///        "not": {
///          "$comment": "enum list is dynamically generated",
///          "enum": [
///            0,
///            2,
///            3,
///            1,
///            4
///          ]
///        }
///      }
///    },
///    "title": "Unknown layer types",
///    "type": "object"
///  },
///  "visual-layer": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/layers/layer"
///      },
///      {
///        "properties": {
///          "ao": {
///            "$ref": "#/$defs/values/int-boolean",
///            "default": 0,
///            "description": "If 1, the layer will rotate itself to match its animated position path",
///            "title": "Auto Orient"
///          },
///          "ks": {
///            "$ref": "#/$defs/helpers/transform",
///            "description": "Layer transform",
///            "title": "Transform"
///          },
///          "masksProperties": {
///            "description": "Optional array of masks for the layer.",
///            "items": {
///              "$ref": "#/$defs/helpers/mask"
///            },
///            "title": "Masks",
///            "type": "array"
///          },
///          "tp": {
///            "description": "Index of the layer used as matte, if omitted assume the layer above the current one",
///            "title": "Matte Parent",
///            "type": "integer"
///          },
///          "tt": {
///            "$ref": "#/$defs/constants/matte-mode",
///            "description": "Defines the track matte mode for the layer",
///            "title": "Matte Mode"
///          }
///        },
///        "required": [
///          "ks"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Layer used to affect visual elements",
///    "title": "Visual Layer",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Layers(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Layers {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Layers { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Layers {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Layers {
    #[inline]
    fn clone(&self) -> Layers {
        Layers(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Layers {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Layers", &&self.0)
    }
}
impl ::std::ops::Deref for Layers {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Layers> for ::serde_json::Value {
    fn from(value: Layers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Layers> for Layers {
    fn from(value: &Layers) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Layers {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Properties`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "base-keyframe": {
///    "allOf": [
///      {
///        "properties": {
///          "h": {
///            "$ref": "#/$defs/values/int-boolean",
///            "default": 0,
///            "title": "Hold"
///          },
///          "i": {
///            "$ref": "#/$defs/properties/easing-handle",
///            "description": "Easing tangent going into the next keyframe",
///            "title": "In Tangent"
///          },
///          "o": {
///            "$ref": "#/$defs/properties/easing-handle",
///            "description": "Easing tangent leaving the current keyframe",
///            "title": "Out Tangent"
///          },
///          "t": {
///            "default": 0,
///            "description": "Frame number",
///            "title": "Time",
///            "type": "number"
///          }
///        }
///      }
///    ],
///    "description": "A Keyframes specifies the value at a specific time and the interpolation function to reach the next keyframe.",
///    "required": [
///      "t"
///    ],
///    "title": "Base Keyframe",
///    "type": "object"
///  },
///  "bezier-keyframe": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/properties/base-keyframe"
///      },
///      {
///        "properties": {
///          "s": {
///            "description": "Value at this keyframe.",
///            "items": {
///              "$ref": "#/$defs/values/bezier"
///            },
///            "maxItems": 1,
///            "minItems": 1,
///            "title": "Value",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "required": [
///      "s"
///    ],
///    "title": "Shape Keyframe",
///    "type": "object"
///  },
///  "bezier-property": {
///    "description": "An animatable property that holds a Bezier shape",
///    "oneOf": [
///      {
///        "$comment": "Not animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 0,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "$ref": "#/$defs/values/bezier",
///            "description": "Static Value",
///            "title": "Value"
///          }
///        }
///      },
///      {
///        "$comment": "Animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 1,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Array of keyframes",
///            "items": {
///              "$ref": "#/$defs/properties/bezier-keyframe"
///            },
///            "title": "Keyframes",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "required": [
///      "a",
///      "k"
///    ],
///    "title": "Bezier Property",
///    "type": "object"
///  },
///  "color-keyframe": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/properties/base-keyframe"
///      },
///      {
///        "properties": {
///          "s": {
///            "$ref": "#/$defs/values/color",
///            "description": "Value at this keyframe.",
///            "title": "Value"
///          }
///        }
///      }
///    ],
///    "required": [
///      "s"
///    ],
///    "title": "Color Keyframe",
///    "type": "object"
///  },
///  "color-property": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/slottable-property"
///      }
///    ],
///    "description": "An animatable property that holds a Color",
///    "oneOf": [
///      {
///        "$comment": "Not animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 0,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "$ref": "#/$defs/values/color",
///            "description": "Static Value",
///            "title": "Value"
///          }
///        }
///      },
///      {
///        "$comment": "Animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 1,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Array of keyframes",
///            "items": {
///              "$ref": "#/$defs/properties/color-keyframe"
///            },
///            "title": "Keyframes",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "title": "Color Property",
///    "type": "object"
///  },
///  "easing-handle": {
///    "description": "Bezier handle for keyframe interpolation",
///    "properties": {
///      "x": {
///        "description": "Time component:\n0 means start time of the keyframe,\n1 means time of the next keyframe.",
///        "oneOf": [
///          {
///            "$ref": "#/$defs/values/vector",
///            "items": {
///              "default": 0,
///              "maximum": 1,
///              "minimum": 0,
///              "type": "number"
///            },
///            "minItems": 1,
///            "type": "array"
///          },
///          {
///            "default": 0,
///            "maximum": 1,
///            "minimum": 0,
///            "type": "number"
///          }
///        ],
///        "title": "X"
///      },
///      "y": {
///        "description": "Value interpolation component:\n0 means start value of the keyframe,\n1 means value at the next keyframe.",
///        "oneOf": [
///          {
///            "$ref": "#/$defs/values/vector",
///            "items": {
///              "default": 0,
///              "type": "number"
///            },
///            "minItems": 1,
///            "type": "array"
///          },
///          {
///            "default": 0,
///            "type": "number"
///          }
///        ],
///        "title": "Y"
///      }
///    },
///    "required": [
///      "x",
///      "y"
///    ],
///    "title": "Keyframe Easing",
///    "type": "object"
///  },
///  "gradient-keyframe": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/properties/base-keyframe"
///      },
///      {
///        "properties": {
///          "s": {
///            "$ref": "#/$defs/values/gradient",
///            "description": "Value at this keyframe.",
///            "title": "Value"
///          }
///        }
///      }
///    ],
///    "required": [
///      "s"
///    ],
///    "title": "Gradient Keyframe",
///    "type": "object"
///  },
///  "gradient-property": {
///    "description": "An animatable property that holds a Gradient",
///    "properties": {
///      "k": {
///        "description": "Animatable vector representing the gradient stops",
///        "oneOf": [
///          {
///            "$comment": "Not animated",
///            "properties": {
///              "a": {
///                "$ref": "#/$defs/values/int-boolean",
///                "const": 0,
///                "description": "Whether the property is animated",
///                "title": "Animated"
///              },
///              "k": {
///                "$ref": "#/$defs/values/gradient",
///                "description": "Static Value",
///                "title": "Value"
///              }
///            }
///          },
///          {
///            "$comment": "Animated",
///            "properties": {
///              "a": {
///                "$ref": "#/$defs/values/int-boolean",
///                "const": 1,
///                "description": "Whether the property is animated",
///                "title": "Animated"
///              },
///              "k": {
///                "description": "Array of keyframes",
///                "items": {
///                  "$ref": "#/$defs/properties/gradient-keyframe"
///                },
///                "title": "Keyframes",
///                "type": "array"
///              }
///            }
///          }
///        ],
///        "required": [
///          "a",
///          "k"
///        ],
///        "title": "Gradient stops",
///        "type": "object"
///      },
///      "p": {
///        "title": "Color stop count",
///        "type": "number"
///      }
///    },
///    "title": "Gradient Property",
///    "type": "object"
///  },
///  "position-keyframe": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/properties/vector-keyframe"
///      },
///      {
///        "properties": {
///          "ti": {
///            "$ref": "#/$defs/values/vector",
///            "description": "Tangent for values (eg: moving position around a curved path)",
///            "title": "Value In Tangent"
///          },
///          "to": {
///            "$ref": "#/$defs/values/vector",
///            "description": "Tangent for values (eg: moving position around a curved path)",
///            "title": "Value Out Tangent"
///          }
///        }
///      }
///    ],
///    "title": "Position Keyframe",
///    "type": "object"
///  },
///  "position-property": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/slottable-property"
///      }
///    ],
///    "description": "An animatable property to represent a position in space",
///    "oneOf": [
///      {
///        "$comment": "Not animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 0,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "$ref": "#/$defs/values/vector",
///            "description": "Static Value",
///            "title": "Value"
///          }
///        }
///      },
///      {
///        "$comment": "Animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 1,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Array of keyframes",
///            "items": {
///              "$ref": "#/$defs/properties/position-keyframe"
///            },
///            "title": "Keyframes",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "required": [
///      "a",
///      "k"
///    ],
///    "title": "Position Property",
///    "type": "object"
///  },
///  "scalar-property": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/slottable-property"
///      }
///    ],
///    "description": "An animatable property that holds a float",
///    "oneOf": [
///      {
///        "$comment": "Not animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 0,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Static Value",
///            "title": "Value",
///            "type": "number"
///          }
///        }
///      },
///      {
///        "$comment": "Animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 1,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Array of keyframes",
///            "items": {
///              "$ref": "#/$defs/properties/vector-keyframe"
///            },
///            "title": "Keyframes",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "title": "Scalar Property",
///    "type": "object"
///  },
///  "split-position": {
///    "description": "An animatable position where x and y are defined and animated separately.",
///    "properties": {
///      "s": {
///        "const": true,
///        "description": "Whether the position has split values",
///        "title": "Split",
///        "type": "boolean"
///      },
///      "x": {
///        "$ref": "#/$defs/properties/scalar-property",
///        "description": "X Position",
///        "title": "X Position"
///      },
///      "y": {
///        "$ref": "#/$defs/properties/scalar-property",
///        "description": "Y Position",
///        "title": "Y Position"
///      }
///    },
///    "required": [
///      "s",
///      "x",
///      "y"
///    ],
///    "title": "Split Position",
///    "type": "object"
///  },
///  "splittable-position-property": {
///    "description": "An animatable position where position values may be defined and animated separately.",
///    "oneOf": [
///      {
///        "$comment": "Grouped XY position coordinates",
///        "$ref": "#/$defs/properties/position-property",
///        "properties": {
///          "s": {
///            "const": false,
///            "description": "Whether the position has split values",
///            "title": "Split",
///            "type": "boolean"
///          }
///        }
///      },
///      {
///        "$comment": "Split XY position coordinates",
///        "$ref": "#/$defs/properties/split-position"
///      }
///    ],
///    "title": "Splittable Position Property",
///    "type": "object"
///  },
///  "vector-keyframe": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/properties/base-keyframe"
///      },
///      {
///        "properties": {
///          "s": {
///            "$ref": "#/$defs/values/vector",
///            "description": "Value at this keyframe.",
///            "title": "Value"
///          }
///        }
///      }
///    ],
///    "required": [
///      "s"
///    ],
///    "title": "Vector Keyframe",
///    "type": "object"
///  },
///  "vector-property": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/slottable-property"
///      }
///    ],
///    "description": "An animatable property that holds an array of numbers",
///    "oneOf": [
///      {
///        "$comment": "Not animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 0,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "$ref": "#/$defs/values/vector",
///            "description": "Static Value",
///            "title": "Value"
///          }
///        }
///      },
///      {
///        "$comment": "Animated",
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/values/int-boolean",
///            "const": 1,
///            "description": "Whether the property is animated",
///            "title": "Animated"
///          },
///          "k": {
///            "description": "Array of keyframes",
///            "items": {
///              "$ref": "#/$defs/properties/vector-keyframe"
///            },
///            "title": "Keyframes",
///            "type": "array"
///          }
///        }
///      }
///    ],
///    "title": "Vector Property",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Properties(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Properties {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Properties { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Properties {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Properties {
    #[inline]
    fn clone(&self) -> Properties {
        Properties(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Properties {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Properties", &&self.0)
    }
}
impl ::std::ops::Deref for Properties {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Properties> for ::serde_json::Value {
    fn from(value: Properties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Properties> for Properties {
    fn from(value: &Properties) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Properties {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Shapes`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "all-graphic-elements": {
///    "$comment": "List of valid shapes",
///    "oneOf": [
///      {
///        "$ref": "#/$defs/shapes/ellipse"
///      },
///      {
///        "$ref": "#/$defs/shapes/fill"
///      },
///      {
///        "$ref": "#/$defs/shapes/gradient-fill"
///      },
///      {
///        "$ref": "#/$defs/shapes/gradient-stroke"
///      },
///      {
///        "$ref": "#/$defs/shapes/group"
///      },
///      {
///        "$ref": "#/$defs/shapes/path"
///      },
///      {
///        "$ref": "#/$defs/shapes/polystar"
///      },
///      {
///        "$ref": "#/$defs/shapes/rectangle"
///      },
///      {
///        "$ref": "#/$defs/shapes/stroke"
///      },
///      {
///        "$ref": "#/$defs/shapes/transform"
///      },
///      {
///        "$ref": "#/$defs/shapes/trim-path"
///      },
///      {
///        "$ref": "#/$defs/shapes/unknown-shape"
///      }
///    ]
///  },
///  "base-gradient": {
///    "allOf": [
///      {
///        "properties": {
///          "a": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Highlight Angle in clockwise degrees, relative to the direction from `s` to `e`",
///            "title": "Highlight Angle"
///          },
///          "e": {
///            "$ref": "#/$defs/properties/position-property",
///            "description": "End point for the gradient",
///            "title": "End Point"
///          },
///          "g": {
///            "$ref": "#/$defs/properties/gradient-property",
///            "description": "Gradient colors",
///            "title": "Colors"
///          },
///          "h": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Highlight Length, as a percentage between `s` and `e`",
///            "title": "Highlight Length"
///          },
///          "s": {
///            "$ref": "#/$defs/properties/position-property",
///            "description": "Starting point for the gradient",
///            "title": "Start Point"
///          },
///          "t": {
///            "$ref": "#/$defs/constants/gradient-type",
///            "description": "Type of the gradient",
///            "title": "Gradient Type"
///          }
///        },
///        "required": [
///          "s",
///          "e",
///          "g",
///          "t"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Common properties for gradients",
///    "title": "Base Gradient",
///    "type": "object"
///  },
///  "base-stroke": {
///    "allOf": [
///      {
///        "properties": {
///          "d": {
///            "description": "Dashed line definition",
///            "items": {
///              "$ref": "#/$defs/shapes/stroke-dash"
///            },
///            "title": "Dashes",
///            "type": "array"
///          },
///          "lc": {
///            "$ref": "#/$defs/constants/line-cap",
///            "default": 2,
///            "title": "Line Cap"
///          },
///          "lj": {
///            "$ref": "#/$defs/constants/line-join",
///            "default": 2,
///            "title": "Line Join"
///          },
///          "ml": {
///            "default": 0,
///            "title": "Miter Limit",
///            "type": "number"
///          },
///          "ml2": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Animatable alternative to ml",
///            "title": "Miter Limit"
///          },
///          "w": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Stroke width",
///            "title": "Width"
///          }
///        },
///        "required": [
///          "w"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Common properties for stroke styles",
///    "title": "Base Stroke",
///    "type": "object"
///  },
///  "ellipse": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape"
///      },
///      {
///        "properties": {
///          "p": {
///            "$ref": "#/$defs/properties/position-property",
///            "title": "Position"
///          },
///          "s": {
///            "$ref": "#/$defs/properties/vector-property",
///            "title": "Size"
///          },
///          "ty": {
///            "const": "el",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "s",
///          "p"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Ellipse shape",
///    "title": "Ellipse",
///    "type": "object"
///  },
///  "fill": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape-style"
///      },
///      {
///        "properties": {
///          "c": {
///            "$ref": "#/$defs/properties/color-property",
///            "title": "Color"
///          },
///          "r": {
///            "$ref": "#/$defs/constants/fill-rule",
///            "title": "Fill Rule"
///          },
///          "ty": {
///            "const": "fl",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "c"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Solid fill color",
///    "title": "Fill",
///    "type": "object"
///  },
///  "gradient-fill": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape-style"
///      },
///      {
///        "$ref": "#/$defs/shapes/base-gradient"
///      },
///      {
///        "properties": {
///          "r": {
///            "$ref": "#/$defs/constants/fill-rule",
///            "title": "Fill Rule"
///          },
///          "ty": {
///            "const": "gf",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Gradient fill color",
///    "title": "Gradient",
///    "type": "object"
///  },
///  "gradient-stroke": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape-style"
///      },
///      {
///        "$ref": "#/$defs/shapes/base-stroke"
///      },
///      {
///        "$ref": "#/$defs/shapes/base-gradient"
///      },
///      {
///        "properties": {
///          "ty": {
///            "const": "gs",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Gradient stroke",
///    "title": "Gradient Stroke",
///    "type": "object"
///  },
///  "graphic-element": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/visual-object"
///      },
///      {
///        "properties": {
///          "hd": {
///            "description": "Whether the shape is hidden",
///            "title": "Hidden",
///            "type": "boolean"
///          },
///          "ty": {
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Element used to display vector data in a shape layer",
///    "title": "Graphic Element",
///    "type": "object"
///  },
///  "group": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/graphic-element"
///      },
///      {
///        "properties": {
///          "it": {
///            "items": {
///              "$ref": "#/$defs/shapes/all-graphic-elements"
///            },
///            "title": "Shapes",
///            "type": "array"
///          },
///          "np": {
///            "title": "Number Of Properties",
///            "type": "number"
///          },
///          "ty": {
///            "const": "gr",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Shape Element that can contain other shapes",
///    "title": "Group",
///    "type": "object"
///  },
///  "modifier": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/graphic-element"
///      }
///    ],
///    "description": "Modifiers change the bezier curves of neighbouring shapes",
///    "title": "Modifier",
///    "type": "object"
///  },
///  "path": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape"
///      },
///      {
///        "properties": {
///          "ks": {
///            "$ref": "#/$defs/properties/bezier-property",
///            "description": "Bezier path",
///            "title": "Shape"
///          },
///          "ty": {
///            "const": "sh",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "ks"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Custom Bezier shape",
///    "title": "Path",
///    "type": "object"
///  },
///  "polystar": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape"
///      },
///      {
///        "properties": {
///          "ir": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "title": "Inner Radius"
///          },
///          "is": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Inner Roundness as a percentage",
///            "title": "Inner Roundness"
///          },
///          "or": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "title": "Outer Radius"
///          },
///          "os": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Outer Roundness as a percentage",
///            "title": "Outer Roundness"
///          },
///          "p": {
///            "$ref": "#/$defs/properties/position-property",
///            "title": "Position"
///          },
///          "pt": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "title": "Points"
///          },
///          "r": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Rotation, clockwise in degrees",
///            "title": "Rotation"
///          },
///          "sy": {
///            "$ref": "#/$defs/constants/star-type",
///            "default": 1,
///            "title": "Star Type"
///          },
///          "ty": {
///            "const": "sr",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "or",
///          "os",
///          "pt",
///          "p",
///          "r"
///        ],
///        "type": "object"
///      },
///      {
///        "if": {
///          "properties": {
///            "sy": {
///              "const": 1
///            }
///          }
///        },
///        "then": {
///          "required": [
///            "ir",
///            "is"
///          ]
///        }
///      }
///    ],
///    "description": "Star or regular polygon",
///    "title": "PolyStar",
///    "type": "object"
///  },
///  "rectangle": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape"
///      },
///      {
///        "properties": {
///          "p": {
///            "$ref": "#/$defs/properties/position-property",
///            "description": "Center of the rectangle",
///            "title": "Position"
///          },
///          "r": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Rounded corners radius",
///            "title": "Rounded"
///          },
///          "s": {
///            "$ref": "#/$defs/properties/vector-property",
///            "title": "Size"
///          },
///          "ty": {
///            "const": "rc",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "s",
///          "p"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "A simple rectangle shape",
///    "title": "Rectangle",
///    "type": "object"
///  },
///  "shape": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/graphic-element"
///      },
///      {
///        "properties": {
///          "d": {
///            "$ref": "#/$defs/constants/shape-direction",
///            "description": "Direction the shape is drawn as, mostly relevant when using trim path",
///            "title": "Direction"
///          }
///        },
///        "type": "object"
///      }
///    ],
///    "description": "Drawable shape, defines the actual shape but not the style",
///    "title": "Shape",
///    "type": "object"
///  },
///  "shape-style": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/graphic-element"
///      },
///      {
///        "properties": {
///          "o": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Opacity, 100 means fully opaque",
///            "title": "Opacity"
///          }
///        },
///        "required": [
///          "o"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Describes the visual appearance (like fill and stroke) of neighbouring shapes",
///    "title": "Shape Style",
///    "type": "object"
///  },
///  "stroke": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/shape-style"
///      },
///      {
///        "$ref": "#/$defs/shapes/base-stroke"
///      },
///      {
///        "properties": {
///          "c": {
///            "$ref": "#/$defs/properties/color-property",
///            "description": "Stroke color",
///            "title": "Color"
///          },
///          "ty": {
///            "const": "st",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "c"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Solid stroke",
///    "title": "Stroke",
///    "type": "object"
///  },
///  "stroke-dash": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/helpers/visual-object"
///      },
///      {
///        "properties": {
///          "n": {
///            "$ref": "#/$defs/constants/stroke-dash-type",
///            "default": "d",
///            "title": "Dash Type"
///          },
///          "v": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Length of the dash",
///            "title": "Length"
///          }
///        },
///        "required": [],
///        "type": "object"
///      }
///    ],
///    "description": "An item used to described the dash pattern in a stroked path",
///    "title": "Stroke Dash",
///    "type": "object"
///  },
///  "transform": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/graphic-element"
///      },
///      {
///        "$ref": "#/$defs/helpers/transform"
///      },
///      {
///        "properties": {
///          "ty": {
///            "const": "tr",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Group transform",
///    "title": "Transform Shape",
///    "type": "object"
///  },
///  "trim-path": {
///    "allOf": [
///      {
///        "$ref": "#/$defs/shapes/modifier"
///      },
///      {
///        "properties": {
///          "e": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Segment end",
///            "title": "End"
///          },
///          "m": {
///            "$ref": "#/$defs/constants/trim-multiple-shapes",
///            "description": "How to treat multiple copies",
///            "title": "Multiple"
///          },
///          "o": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "title": "Offset"
///          },
///          "s": {
///            "$ref": "#/$defs/properties/scalar-property",
///            "description": "Segment start",
///            "title": "Start"
///          },
///          "ty": {
///            "const": "tm",
///            "title": "Shape Type",
///            "type": "string"
///          }
///        },
///        "required": [
///          "ty",
///          "o",
///          "s",
///          "e"
///        ],
///        "type": "object"
///      }
///    ],
///    "description": "Trims shapes into a segment",
///    "title": "Trim Path",
///    "type": "object"
///  },
///  "unknown-shape": {
///    "description": "Unknown shape types. Types not defined by the specification are still allowed.",
///    "properties": {
///      "ty": {
///        "not": {
///          "$comment": "enum list is dynamically generated",
///          "enum": [
///            "el",
///            "fl",
///            "gf",
///            "gs",
///            "gr",
///            "sh",
///            "sr",
///            "rc",
///            "st",
///            "tr",
///            "tm"
///          ]
///        }
///      }
///    },
///    "title": "Unknown shape types",
///    "type": "object"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Shapes(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Shapes {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Shapes { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Shapes {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Shapes {
    #[inline]
    fn clone(&self) -> Shapes {
        Shapes(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Shapes {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Shapes", &&self.0)
    }
}
impl ::std::ops::Deref for Shapes {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Shapes> for ::serde_json::Value {
    fn from(value: Shapes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Shapes> for Shapes {
    fn from(value: &Shapes) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Shapes {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
///`Values`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "bezier": {
///    "description": "Cubic polybezier",
///    "properties": {
///      "c": {
///        "default": false,
///        "title": "Closed",
///        "type": "boolean"
///      },
///      "i": {
///        "description": "Array of points, each point is an array of coordinates.\nThese points are along the `in` tangents relative to the corresponding `v`.",
///        "items": {
///          "$ref": "#/$defs/values/vector",
///          "default": []
///        },
///        "title": "In Tangents",
///        "type": "array"
///      },
///      "o": {
///        "description": "Array of points, each point is an array of coordinates.\nThese points are along the `out` tangents relative to the corresponding `v`.",
///        "items": {
///          "$ref": "#/$defs/values/vector",
///          "default": []
///        },
///        "title": "Out Tangents",
///        "type": "array"
///      },
///      "v": {
///        "description": "Array of points, each point is an array of coordinates.\nThese points are along the bezier path",
///        "items": {
///          "$ref": "#/$defs/values/vector",
///          "default": []
///        },
///        "title": "Vertices",
///        "type": "array"
///      }
///    },
///    "required": [
///      "i",
///      "v",
///      "o"
///    ],
///    "title": "Bezier",
///    "type": "object"
///  },
///  "color": {
///    "description": "Color as a [r, g, b] array with values in [0, 1]",
///    "items": {
///      "maximum": 1,
///      "minimum": 0,
///      "type": "number"
///    },
///    "maxItems": 4,
///    "minItems": 3,
///    "title": "Color",
///    "type": "array"
///  },
///  "data-url": {
///    "description": "An embedded data object",
///    "pattern": "^data:([\\w/]+)(;base64)?,(.+)$",
///    "title": "Data URL",
///    "type": "string"
///  },
///  "gradient": {
///    "description": "A flat list of color stops followed by optional transparency stops. A color stop is [offset, red, green, blue]. A transparency stop is [offset, transparency]. All values are between 0 and 1",
///    "items": {
///      "maximum": 1,
///      "minimum": 0,
///      "type": "number"
///    },
///    "title": "Gradient",
///    "type": "array"
///  },
///  "hexcolor": {
///    "description": "Color value in hexadecimal format, with two digits per component ('#RRGGBB')",
///    "examples": [
///      "#FF00AA"
///    ],
///    "pattern": "^#([a-fA-F0-9]{6})$",
///    "title": "Hex Color",
///    "type": "string"
///  },
///  "int-boolean": {
///    "default": 0,
///    "description": "Represents boolean values as an integer. `0` is false, `1` is true.",
///    "enum": [
///      0,
///      1
///    ],
///    "examples": [
///      0
///    ],
///    "oneOf": [
///      {
///        "const": 1,
///        "title": "True"
///      },
///      {
///        "const": 0,
///        "title": "False"
///      }
///    ],
///    "title": "Integer Boolean",
///    "type": "integer"
///  },
///  "vector": {
///    "description": "An array of numbers",
///    "items": {
///      "type": "number"
///    },
///    "title": "Vector",
///    "type": "array"
///  }
///}
/// ```
/// </details>
#[serde(transparent)]
pub struct Values(pub ::serde_json::Value);
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Values {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            _serde::__private::Result::map(
                _serde::Deserialize::deserialize(__deserializer),
                |__transparent| Values { 0: __transparent },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Values {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serialize::serialize(&self.0, __serializer)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for Values {
    #[inline]
    fn clone(&self) -> Values {
        Values(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Values {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Values", &&self.0)
    }
}
impl ::std::ops::Deref for Values {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Values> for ::serde_json::Value {
    fn from(value: Values) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Values> for Values {
    fn from(value: &Values) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for Values {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
fn main() {}