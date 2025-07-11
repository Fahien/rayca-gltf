// Copyright © 2022-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default)]
// Model representation based on glTF spec
pub struct Model {
    pub scene: Vec<Handle<Node>>,
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

impl Model {
    pub fn extend(&mut self, other: Model) {
        self.scene.extend(other.scene);
        self.nodes.extend(other.nodes);
        self.meshes.extend(other.meshes);
        self.primitives.extend(other.primitives);
        self.materials.extend(other.materials);
        self.textures.extend(other.textures);
        self.images.extend(other.images);
        self.samplers.extend(other.samplers);
        self.cameras.extend(other.cameras);
        self.scripts.extend(other.scripts);
    }
}
