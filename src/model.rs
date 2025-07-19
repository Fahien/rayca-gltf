// Copyright © 2022-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

// Model representation based on glTF spec
pub struct Model {
    pub name: String,
    pub scene: Node,
    pub buffers: Pack<Buffer>,
    pub buffer_views: Pack<BufferView>,
    pub nodes: Pack<Node>,
    pub meshes: Pack<Mesh>,
    pub primitives: Pack<Primitive>,
    pub materials: Pack<Material>,
    pub textures: Pack<Texture>,
    pub images: Pack<Image>,
    pub samplers: Pack<Sampler>,
    pub cameras: Pack<Camera>,
    pub scripts: Pack<Script>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            scene: Node::default(),
            buffers: Pack::default(),
            buffer_views: Pack::default(),
            nodes: Pack::default(),
            meshes: Pack::default(),
            primitives: Pack::default(),
            materials: Pack::default(),
            textures: Pack::default(),
            images: Pack::default(),
            samplers: Pack::default(),
            cameras: Pack::default(),
            scripts: Pack::default(),
        }
    }
}
