// Copyright © 2021-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Clone)]
pub struct NodeBuilder {
    name: String,
    trs: Trs,
    children: Vec<Handle<Node>>,
    mesh: Handle<Mesh>,
    camera: Handle<Camera>,
    script: Handle<Script>,
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            trs: Trs::default(),
            children: Vec::new(),
            mesh: Handle::default(),
            camera: Handle::default(),
            script: Handle::default(),
        }
    }
}

impl NodeBuilder {
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn trs(mut self, trs: Trs) -> Self {
        self.trs = trs;
        self
    }

    pub fn children(mut self, children: Vec<Handle<Node>>) -> Self {
        self.children = children;
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

    pub fn script(mut self, script: Handle<Script>) -> Self {
        self.script = script;
        self
    }

    pub fn build(self) -> Node {
        Node {
            name: self.name,
            trs: self.trs,
            children: self.children,
            mesh: self.mesh,
            camera: self.camera,
            script: self.script,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub trs: Trs,
    pub children: Vec<Handle<Node>>,
    pub mesh: Handle<Mesh>,
    pub camera: Handle<Camera>,
    pub script: Handle<Script>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            trs: Trs::default(),
            children: Vec::new(),
            mesh: Handle::default(),
            camera: Handle::default(),
            script: Handle::default(),
        }
    }
}

impl Node {
    pub fn builder() -> NodeBuilder {
        NodeBuilder::default()
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"name\": \"{}\", \"translation\": {}, \"rotation\": {}, \"scale\": {}",
            self.name, self.trs.translation, self.trs.rotation, self.trs.scale
        )?;
        if self.camera.is_valid() {
            write!(f, ", \"camera\": {}", self.camera.id)?;
        }
        if self.mesh.is_valid() {
            write!(f, ", \"mesh\": {}", self.mesh.id)?;
        }
        if !self.children.is_empty() {
            write!(f, ", \"children\": [")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", child.id)?;
            }
            write!(f, "]")?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}
