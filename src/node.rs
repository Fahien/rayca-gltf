// Copyright © 2021-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Clone, Default)]
pub struct NodeBuilder {
    trs: Trs,
    mesh: Handle<Mesh>,
    camera: Handle<Camera>,
}

impl NodeBuilder {
    pub fn trs(mut self, trs: Trs) -> Self {
        self.trs = trs;
        self
    }

    pub fn mesh(mut self, mesh: Handle<Mesh>) -> Self {
        self.mesh = mesh;
        self
    }

    pub fn camera(mut self, camera: Handle<Camera>) -> Self {
        self.camera = camera;
        self
    }

    pub fn build(self) -> Node {
        Node {
            trs: self.trs,
            mesh: self.mesh,
            camera: self.camera,
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Node {
    pub trs: Trs,
    pub children: Vec<Handle<Node>>,
    pub mesh: Handle<Mesh>,
    pub camera: Handle<Camera>,
}

impl Node {
    pub fn builder() -> NodeBuilder {
        NodeBuilder::default()
    }

    pub fn new() -> Self {
        Self::default()
    }
}
