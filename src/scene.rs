// Copyright © 2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

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
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("."),
            name: "Unknown".to_string(),
            nodes: Pack::default(),
            models: Pack::default(),
        }
    }
}

impl Scene {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let mut scene = Self::default();
        scene.name = name.into();
        scene
    }

    pub fn get_uri(&self) -> PathBuf {
        self.dir.join(format!("{}.glx", self.name))
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
