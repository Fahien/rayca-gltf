// Copyright © 2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use crate::*;

#[derive(Default, Clone)]
pub struct Buffer {
    pub uri: String,
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new(uri: String, data: Vec<u8>) -> Self {
        Self { uri, data }
    }

    pub fn extend_from_bytes<B: AsBytes>(
        &mut self,
        bytes: &B,
        stride: usize,
        target: BufferViewTarget,
    ) -> BufferView {
        let offset = self.data.len();
        self.data.extend_from_slice(bytes.as_bytes());
        // Always align up to 4 bytes
        let padding = 4 - (self.data.len() % 4);
        if padding < 4 {
            self.data.extend(vec![0; padding]);
        }
        let size = self.data.len() - offset;

        BufferView::new(
            0.into(), // Handle<Buffer> is not used here, so we use a dummy value
            offset,
            size,
            stride,
            target,
        )
    }
}

impl std::ops::Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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

impl std::fmt::Display for BufferView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"buffer\": {}, \"byteOffset\": {}, \"byteLength\": {}, \"target\": {}",
            self.buffer.id, self.offset, self.size, self.target as u32
        )?;
        if self.stride > 0 {
            write!(f, ", \"byteStride\": {}", self.stride)?;
        }
        write!(f, " }}")
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

#[derive(Default)]
pub struct Accessor {
    pub buffer_view: Handle<BufferView>,

    /// The offset relative to the start of the buffer view in bytes.
    pub offset: usize,

    /// The datatype of the accessor’s components.
    pub component_type: ComponentType,

    /// The number of elements referenced by this accessor.
    pub count: usize,

    /// Specifies if the accessor’s elements are scalars, vectors, or matrices.
    pub accessor_type: AccessorType,
}

impl Accessor {
    pub fn new(
        buffer_view: Handle<BufferView>,
        offset: usize,
        component_type: ComponentType,
        count: usize,
        accessor_type: AccessorType,
    ) -> Self {
        Self {
            buffer_view,
            offset,
            component_type,
            count,
            accessor_type,
        }
    }
}

impl std::fmt::Display for Accessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"bufferView\": {}, \"byteOffset\": {}, \"componentType\": {}, \"count\": {}, \"type\": \"{}\" }}",
            self.buffer_view.id,
            self.offset,
            self.component_type as u32,
            self.count,
            self.accessor_type
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(u32)]
pub enum ComponentType {
    /// Byte
    I8 = 5120,

    #[default]
    /// Unsigned byte
    U8 = 5121,

    /// Short
    I16 = 5122,

    /// Unsigned short
    U16 = 5123,

    /// Unsigned int
    U32 = 5125,

    /// Float
    F32 = 5126,
}

impl ComponentType {
    pub fn get_size(self) -> usize {
        match self {
            ComponentType::I8 | ComponentType::U8 => 1,
            ComponentType::I16 | ComponentType::U16 => 2,
            ComponentType::U32 | ComponentType::F32 => 4,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AccessorType {
    #[default]
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl std::fmt::Display for AccessorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessorType::Scalar => write!(f, "SCALAR"),
            AccessorType::Vec2 => write!(f, "VEC2"),
            AccessorType::Vec3 => write!(f, "VEC3"),
            AccessorType::Vec4 => write!(f, "VEC4"),
            AccessorType::Mat2 => write!(f, "MAT2"),
            AccessorType::Mat3 => write!(f, "MAT3"),
            AccessorType::Mat4 => write!(f, "MAT4"),
        }
    }
}
