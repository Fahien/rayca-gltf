// Copyright © 2021-2024
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default)]
pub struct MaterialBuilder {
    shader: u32,
    color: Color,
    albedo: Handle<Texture>,
    normal: Handle<Texture>,
    metallic_factor: f32,
    roughness_factor: f32,
    metallic_roughness: Handle<Texture>,
}

impl MaterialBuilder {
    pub fn shader(mut self, shader: u32) -> Self {
        self.shader = shader;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn albedo(mut self, albedo: Handle<Texture>) -> Self {
        self.albedo = albedo;
        self
    }

    pub fn normal(mut self, normal: Handle<Texture>) -> Self {
        self.normal = normal;
        self
    }

    pub fn metallic_factor(mut self, metallic_factor: f32) -> Self {
        self.metallic_factor = metallic_factor;
        self
    }

    pub fn roughness_factor(mut self, roughness_factor: f32) -> Self {
        self.roughness_factor = roughness_factor;
        self
    }

    pub fn metallic_roughness(mut self, metallic_roughness: Handle<Texture>) -> Self {
        self.metallic_roughness = metallic_roughness;
        self
    }

    pub fn build(self) -> Material {
        Material {
            shader: self.shader,
            color: self.color,
            albedo: self.albedo,
            normal: self.normal,
            metallic_factor: self.metallic_factor,
            roughness_factor: self.roughness_factor,
            metallic_roughness: self.metallic_roughness,
        }
    }
}

#[derive(Default, Debug)]
pub struct Material {
    pub shader: u32,
    pub color: Color,
    pub albedo: Handle<Texture>,
    pub normal: Handle<Texture>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness: Handle<Texture>,
}

impl Material {
    pub fn builder() -> MaterialBuilder {
        MaterialBuilder::default()
    }
}
