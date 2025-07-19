// Copyright © 2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default, Clone)]
pub struct Buffer {
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl std::ops::Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Default, Clone)]
pub struct BufferView {
    pub buffer: Handle<Buffer>,
    pub offset: usize,
    pub size: usize,
    pub stride: usize,
    pub target: BufferViewTarget,
}

impl BufferView {
    pub fn new(
        buffer: Handle<Buffer>,
        offset: usize,
        size: usize,
        stride: usize,
        target: BufferViewTarget,
    ) -> Self {
        Self {
            buffer,
            offset,
            size,
            stride,
            target,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BufferViewTarget {
    #[default]
    None = 0,
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

impl From<u32> for BufferViewTarget {
    fn from(value: u32) -> Self {
        match value {
            34962 => BufferViewTarget::ArrayBuffer,
            34963 => BufferViewTarget::ElementArrayBuffer,
            _ => BufferViewTarget::None,
        }
    }
}
