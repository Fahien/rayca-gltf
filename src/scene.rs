// Copyright © 2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use serde::*;

use crate::*;

/// Scene representation
#[derive(Serialize, Deserialize)]
pub struct Scene {
    #[serde(skip)]
    pub dir: PathBuf,
    pub name: String,
    pub nodes: Pack<Node>,
    pub models: Pack<ModelSource>,
    pub root: Node,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("."),
            name: "Unknown".to_string(),
            nodes: Pack::default(),
            models: Pack::default(),
            root: Node::default(),
        }
    }
}

impl Scene {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let mut scene = Self::default();
        scene.name = name.into();
        scene
    }

    pub fn load_glx_path<P: AsRef<Path>>(glx_path: P, assets: &Assets) -> Self {
        let asset = assets.load(glx_path.as_ref());
        let data = asset.to_string();
        serde_json::from_str(&data).expect("Failed to load GLX file")
    }

    pub fn get_uri(&self) -> PathBuf {
        self.dir.join(format!("{}.glx", self.name))
    }

    pub fn get_node(&self, handle: Handle<Node>) -> Option<&Node> {
        self.nodes.get(handle)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModelSource {
    pub uri: String,
}

impl Default for ModelSource {
    fn default() -> Self {
        Self {
            uri: "Unknown".to_string(),
        }
    }
}

impl ModelSource {
    pub fn new<S: Into<String>>(uri: S) -> Self {
        Self { uri: uri.into() }
    }
}
