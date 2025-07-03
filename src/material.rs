// Copyright © 2021-2024
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default)]
pub struct MaterialBuilder {
    shader: u32,
    texture: Handle<Texture>,
}

impl MaterialBuilder {
    pub fn shader(mut self, shader: u32) -> Self {
        self.shader = shader;
        self
    }

    pub fn texture(mut self, texture: Handle<Texture>) -> Self {
        self.texture = texture;
        self
    }

    pub fn build(self) -> Material {
        Material {
            shader: self.shader,
            texture: self.texture,
        }
    }
}

#[derive(Default, Debug)]
pub struct Material {
    pub shader: u32,
    pub texture: Handle<Texture>,
}

impl Material {
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder::default()
    }
}
