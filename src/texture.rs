// Copyright © 2021-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default)]
pub struct Image {}

#[derive(Default)]
pub struct Sampler {}

#[derive(Default)]
pub struct Texture {
    pub view: Handle<Image>,
    pub sampler: Handle<Sampler>,
}

impl Texture {
    pub fn new(view: Handle<Image>, sampler: Handle<Sampler>) -> Self {
        Self { view, sampler }
    }
}
