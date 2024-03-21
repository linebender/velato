// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use self::converters::conv_layer;
use crate::parser::{schema, Animation};
use crate::Composition;
use std::collections::HashMap;

mod builders;
mod converters;
mod defaults;
mod util;

pub fn import_composition(
    source: impl AsRef<[u8]>,
) -> Result<Composition, Box<dyn std::error::Error>> {
    let source = Animation::from_slice(source.as_ref())?;

    let mut target = Composition {
        frames: source.in_point..source.out_point,
        frame_rate: source.frame_rate,
        width: source.width,
        height: source.height,
        assets: Default::default(),
        layers: Default::default(),
    };

    // Collect assets and layers
    let mut idmap: HashMap<usize, usize> = HashMap::default();
    if let Some(assets) = source.assets {
        for asset in assets {
            match asset {
                schema::assets::AnyAsset::Precomposition(precomp) => {
                    idmap.clear();
                    let mut layers = vec![];
                    let mut mask_layer = None;
                    for layer in precomp.composition.layers.iter() {
                        let index = layers.len();
                        if let Some((mut layer, id, mask_blend)) = conv_layer(layer) {
                            if let (Some(mask_blend), Some(mask_layer)) =
                                (mask_blend, mask_layer.take())
                            {
                                layer.mask_layer = Some((mask_blend, mask_layer));
                            }
                            if layer.is_mask {
                                mask_layer = Some(index);
                            }
                            idmap.insert(id, index);
                            layers.push(layer);
                        }
                    }
                    for layer in &mut layers {
                        if let Some(parent) = layer.parent {
                            layer.parent = idmap.get(&parent).copied();
                        }
                    }
                    target.assets.insert(precomp.asset.id.clone(), layers);
                }
                asset => {
                    unimplemented!("asset {:?} is not yet implemented", asset)
                }
            }
        }
    }

    idmap.clear();
    let mut layers = vec![];
    let mut mask_layer = None;
    for layer in &source.layers {
        let index = layers.len();
        if let Some((mut layer, id, mask_blend)) = conv_layer(layer) {
            if let (Some(mask_blend), Some(mask_layer)) = (mask_blend, mask_layer.take()) {
                layer.mask_layer = Some((mask_blend, mask_layer));
            }
            if layer.is_mask {
                mask_layer = Some(index);
            }
            idmap.insert(id, index);
            layers.push(layer);
        }
    }
    for layer in &mut layers {
        if let Some(parent) = layer.parent {
            layer.parent = idmap.get(&parent).copied();
        }
    }
    target.layers = layers;

    Ok(target)
}
