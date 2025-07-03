// Copyright © 2021-2024
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Clone, Default)]
pub struct MeshBuilder {
    primitive: Handle<Primitive>,
}

impl MeshBuilder {
    pub fn primitive(mut self, primitive: Handle<Primitive>) -> Self {
        self.primitive = primitive;
        self
    }

    pub fn build(self) -> Mesh {
        Mesh {
            primitive: self.primitive,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Mesh {
    pub primitive: Handle<Primitive>,
}

impl Mesh {
    pub fn builder() -> MeshBuilder {
        MeshBuilder::default()
    }
}
