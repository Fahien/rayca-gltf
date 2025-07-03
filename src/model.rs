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
}
