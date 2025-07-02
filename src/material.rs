// Copyright © 2021-2024
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

#[derive(Default)]
pub struct MaterialBuilder {
    shader: u32,
}

impl MaterialBuilder {
    pub fn shader(mut self, shader: u32) -> Self {
        self.shader = shader;
        self
    }

    pub fn build(self) -> Material {
        Material {
            shader: self.shader,
        }
    }
}

#[derive(Default)]
pub struct Material {
    pub shader: u32,
}

impl Material {
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder::default()
    }
}
