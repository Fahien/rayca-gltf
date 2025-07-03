// Copyright © 2021-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default, Debug)]
pub struct Image {}

#[derive(Default, Debug)]
pub struct Sampler {}

#[derive(Default, Debug)]
pub struct Texture {
    pub view: Handle<Image>,
    pub sampler: Handle<Sampler>,
}

impl Texture {
    pub fn new(view: Handle<Image>, sampler: Handle<Sampler>) -> Self {
        Self { view, sampler }
    }
}
