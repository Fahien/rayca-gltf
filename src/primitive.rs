// Copyright © 2021-2024
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Clone, Default)]
pub struct PrimitiveBuilder {
    material: Handle<Material>,
}

impl PrimitiveBuilder {
    pub fn material(mut self, material: Handle<Material>) -> Self {
        self.material = material;
        self
    }

    pub fn build(self) -> Primitive {
        Primitive {
            material: self.material,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Primitive {
    pub material: Handle<Material>,
}

impl Primitive {
    pub fn builder() -> PrimitiveBuilder {
        PrimitiveBuilder::default()
    }
}
