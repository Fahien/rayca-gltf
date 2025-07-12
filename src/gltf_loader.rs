// Copyright © 2022-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use std::{error::Error, path::Path};

use base64::Engine;
use rayca_math::Trs;

use crate::*;

fn data_type_as_size(data_type: gltf::accessor::DataType) -> usize {
    match data_type {
        gltf::accessor::DataType::I8 => 1,
        gltf::accessor::DataType::U8 => 1,
        gltf::accessor::DataType::I16 => 2,
        gltf::accessor::DataType::U16 => 2,
        gltf::accessor::DataType::U32 => 4,
        gltf::accessor::DataType::F32 => 4,
    }
}

fn dimensions_as_size(dimensions: gltf::accessor::Dimensions) -> usize {
    match dimensions {
        gltf::accessor::Dimensions::Scalar => 1,
        gltf::accessor::Dimensions::Vec2 => 2,
        gltf::accessor::Dimensions::Vec3 => 3,
        gltf::accessor::Dimensions::Vec4 => 4,
        gltf::accessor::Dimensions::Mat2 => 4,
        gltf::accessor::Dimensions::Mat3 => 9,
        gltf::accessor::Dimensions::Mat4 => 16,
    }
}

fn get_stride(accessor: &gltf::Accessor) -> usize {
    if let Some(view) = accessor.view() {
        if let Some(stride) = view.stride() {
            return stride;
        }
    }

    data_type_as_size(accessor.data_type()) * dimensions_as_size(accessor.dimensions())
}

impl From<gltf::accessor::DataType> for ComponentType {
    fn from(value: gltf::accessor::DataType) -> Self {
        match value {
            gltf::accessor::DataType::I8 => ComponentType::I8,
            gltf::accessor::DataType::U8 => ComponentType::U8,
            gltf::accessor::DataType::I16 => ComponentType::I16,
            gltf::accessor::DataType::U16 => ComponentType::U16,
            gltf::accessor::DataType::U32 => ComponentType::U32,
            gltf::accessor::DataType::F32 => ComponentType::F32,
        }
    }
}

struct UriBuffers {
    data: Vec<Vec<u8>>,
}

impl UriBuffers {
    fn new(
        gltf: &gltf::Gltf,
        parent_dir: Option<&Path>,
        assets: &Assets,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            data: Self::load_uri_buffers(gltf, parent_dir, assets)?,
        })
    }

    fn load_uri_buffers(
        gltf: &gltf::Gltf,
        parent_dir: Option<&Path>,
        assets: &Assets,
    ) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
        let mut uri_buffers = vec![];
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    const DATA_URI: &str = "data:application/octet-stream;base64,";

                    let data = if uri.starts_with(DATA_URI) {
                        let (_, data_base64) = uri.split_at(DATA_URI.len());
                        base64::engine::general_purpose::STANDARD.decode(data_base64)?
                    } else if let Some(parent_dir) = &parent_dir {
                        let uri = parent_dir.join(uri);
                        assets.load(uri).data
                    } else {
                        assets.load(uri).data
                    };
                    assert_eq!(buffer.index(), uri_buffers.len());
                    uri_buffers.push(data);
                }
                _ => unimplemented!(),
            }
        }

        Ok(uri_buffers)
    }

    fn load_indices(&self, gprimitive: &gltf::Primitive) -> (Vec<u8>, ComponentType) {
        let mut indices = vec![];
        let mut index_type = ComponentType::U8;

        if let Some(accessor) = gprimitive.indices() {
            let data_type = accessor.data_type();
            index_type = data_type.into();

            // Data type can vary
            let data = self.get_data_start(&accessor);
            let d = &data[0];
            let len = accessor.count() * data_type_as_size(data_type);
            // Use bytes regardless of the index data type
            let slice: &[u8] = unsafe { std::slice::from_raw_parts(d as *const u8 as _, len) };
            indices = Vec::from(slice);
        }

        (indices, index_type)
    }

    fn get_slices<'g>(&self, accessor: &'g gltf::Accessor) -> Vec<&'g [f32]> {
        let data_type = accessor.data_type();
        assert!(data_type == gltf::accessor::DataType::F32);

        let count = accessor.count();
        let dimensions = accessor.dimensions();
        let len = match dimensions {
            gltf::accessor::Dimensions::Vec2 => 2,
            gltf::accessor::Dimensions::Vec3 => 3,
            gltf::accessor::Dimensions::Vec4 => 4,
            _ => panic!("Invalid dimensions"),
        };

        let data = self.get_data_start(accessor);
        let stride = get_stride(accessor);

        let mut ret = vec![];

        for i in 0..count {
            let offset = i * stride;
            assert!(offset < data.len());
            let d = &data[offset];
            let slice = unsafe { std::slice::from_raw_parts::<f32>(d as *const u8 as _, len) };
            ret.push(slice);
        }

        ret
    }

    fn load_positions(
        &self,
        vertices: &mut Vec<Vertex>,
        accessor: &gltf::Accessor,
    ) -> Result<(), Box<dyn Error>> {
        let positions = self.get_slices(accessor);
        vertices.resize(positions.len(), Vertex::default());
        for (i, position) in positions.into_iter().enumerate() {
            vertices[i].pos = Point3::new(position[0], position[1], position[2]);
        }
        Ok(())
    }

    fn load_uvs(
        &self,
        vertices: &mut Vec<Vertex>,
        accessor: &gltf::Accessor,
    ) -> Result<(), Box<dyn Error>> {
        let uvs = self.get_slices(accessor);
        vertices.resize(uvs.len(), Vertex::default());
        for (i, uv) in uvs.into_iter().enumerate() {
            vertices[i].ext.uv.x = uv[0];
            vertices[i].ext.uv.y = uv[1];
        }
        Ok(())
    }

    fn load_normals(
        &self,
        vertices: &mut Vec<Vertex>,
        accessor: &gltf::Accessor,
    ) -> Result<(), Box<dyn Error>> {
        let normals = self.get_slices(accessor);
        vertices.resize(normals.len(), Vertex::default());
        for (i, normal) in normals.into_iter().enumerate() {
            vertices[i].ext.normal = Vec3::new(normal[0], normal[1], normal[2]);
        }
        Ok(())
    }

    fn load_colors(
        &self,
        vertices: &mut Vec<Vertex>,
        accessor: &gltf::Accessor,
    ) -> Result<(), Box<dyn Error>> {
        let colors = self.get_slices(accessor);
        vertices.resize(colors.len(), Vertex::default());
        for (i, color) in colors.into_iter().enumerate() {
            vertices[i].ext.color.r = color[0];
            vertices[i].ext.color.g = color[1];
            vertices[i].ext.color.b = color[2];
            vertices[i].ext.color.a = if color.len() == 4 { color[3] } else { 1.0 };
        }
        Ok(())
    }

    fn load_tangents(&self, vertices: &mut [Vertex], accessor: &gltf::Accessor) {
        let data_type = accessor.data_type();
        assert!(data_type == gltf::accessor::DataType::F32);
        let count = accessor.count();
        assert_eq!(vertices.len(), count);
        let dimensions = accessor.dimensions();
        assert!(dimensions == gltf::accessor::Dimensions::Vec4);

        let view = accessor.view().unwrap();
        let target = view.target().unwrap_or(gltf::buffer::Target::ArrayBuffer);
        assert!(target == gltf::buffer::Target::ArrayBuffer);

        let data = self.get_data_start(accessor);
        let stride = get_stride(accessor);

        vertices.iter_mut().enumerate().for_each(|(index, vertex)| {
            let offset = index * stride;
            assert!(offset < data.len());
            let d = &data[offset];
            let tangent = unsafe { std::slice::from_raw_parts::<f32>(d as *const u8 as _, 4) };

            vertex.ext.tangent = Vec3::new(tangent[0], tangent[1], tangent[2]);

            // Compute bitangent as for glTF 2.0 spec
            vertex.ext.bitangent = vertex.ext.normal.cross(&vertex.ext.tangent) * tangent[3];
        });
    }

    fn get_data_start<'b>(&'b self, accessor: &gltf::Accessor) -> &'b [u8] {
        let view = accessor.view().unwrap();
        let view_len = view.length();

        let buffer = view.buffer();
        if let gltf::buffer::Source::Bin = buffer.source() {
            unimplemented!()
        }

        let view_offset = view.offset();
        let accessor_offset = accessor.offset();
        let offset = accessor_offset + view_offset;
        assert!(offset < buffer.length());
        let end_offset = view_offset + view_len;
        assert!(end_offset <= buffer.length());

        let data = &self.data[buffer.index()];
        &data[offset..end_offset]
    }
}

impl Model {
    fn load_gltf(
        gltf: gltf::Gltf,
        path: Option<&Path>,
        assets: &Assets,
    ) -> Result<Self, Box<dyn Error>> {
        let mut timer = Timer::new();

        let path_str = if let Some(path) = path {
            path.to_string_lossy().to_string()
        } else {
            String::new()
        };

        let parent_dir = if let Some(path) = path {
            Some(path.parent().ok_or("Failed to get parent directory")?)
        } else {
            None
        };

        let mut ret = Self::default();
        ret.load_images(&gltf, parent_dir);
        ret.load_textures(&gltf);
        ret.load_materials(&gltf)?;
        let uri_buffers = UriBuffers::new(&gltf, parent_dir, assets)?;
        ret.load_meshes(&gltf, &uri_buffers)?;
        ret.load_cameras(&gltf);
        ret.load_nodes(&gltf);

        println!(
            "Loaded {} in {:.2}ms",
            path_str,
            timer.get_delta().as_millis()
        );
        Ok(ret)
    }

    pub fn load_gltf_path<P: AsRef<Path>>(
        path: P,
        assets: &Assets,
    ) -> Result<Self, Box<dyn Error>> {
        let data = assets.load(path.as_ref()).data;
        let gltf = gltf::Gltf::from_slice(&data)?;
        Self::load_gltf(gltf, Some(path.as_ref()), assets)
    }

    pub fn load_gltf_data(data: &[u8], assets: &Assets) -> Result<Self, Box<dyn Error>> {
        let gltf = gltf::Gltf::from_slice(data)?;
        Self::load_gltf(gltf, None, assets)
    }

    pub fn load_images(&mut self, gltf: &gltf::Gltf, parent_dir: Option<&Path>) {
        let mut timer = Timer::new();

        let mut vec: Vec<Image> = vec![];

        for image in gltf.images() {
            match image.source() {
                gltf::image::Source::View { .. } => todo!("Implement image source view"),
                gltf::image::Source::Uri { uri, .. } => {
                    const DATA_URI: &str = "data:image/png;base64,";

                    let image = if uri.starts_with(DATA_URI) {
                        let (_, data_base64) = uri.split_at(DATA_URI.len());
                        let _data = base64::engine::general_purpose::STANDARD
                            .decode(data_base64)
                            .expect("Failed to decode base64 image data");
                        unimplemented!("Add support for data uri");
                        //  TODO: what ? buffer.load_png_data(&data)
                    } else if let Some(parent_dir) = &parent_dir {
                        // Join gltf parent dir to URI
                        let path = parent_dir.join(uri);
                        Image::builder().uri(path.to_string_lossy()).build()
                    } else {
                        Image::builder().uri(uri.to_string()).build()
                    };

                    //image.id = id;
                    vec.push(image)
                }
            }
        }

        //vec.sort_by_key(|image| image.id);

        println!(
            "Loaded images from file in {:.2}s",
            timer.get_delta().as_secs_f32()
        );

        self.images = Pack::from(vec);
    }

    pub fn load_textures(&mut self, gltf: &gltf::Gltf) {
        let vec: Vec<Texture> = gltf
            .textures()
            .map(|gtexture| {
                let image = Handle::from(gtexture.source().index());
                let sampler = Handle::NONE;
                Texture::new(image, sampler)
            })
            .collect();
        self.textures = Pack::from(vec);
    }

    pub fn load_materials(&mut self, gltf: &gltf::Gltf) -> Result<(), Box<dyn Error>> {
        for gmaterial in gltf.materials() {
            // If this is a Phong material, its definition is in extras
            // if let Some(extras) = gmaterial.extras() {
            //     if let Ok(material) = serde_json::from_str::<PhongMaterial>(extras.get()) {
            //         self.phong_materials.push(material);
            //         continue;
            //     }
            // }

            // Default behaviour is parsing a GGX material
            let mut material = Material::builder();

            let pbr = gmaterial.pbr_metallic_roughness();

            // Load base color
            let gcolor = pbr.base_color_factor();
            let color = Color::new(gcolor[0], gcolor[1], gcolor[2], gcolor[3]);
            material = material.color(color);

            // Load albedo
            if let Some(gtexture) = pbr.base_color_texture() {
                material = material.albedo(Handle::from(gtexture.texture().index()));
            }

            // Load normal
            if let Some(gtexture) = gmaterial.normal_texture() {
                material = material.normal(Handle::from(gtexture.texture().index()));
            }

            // Load metallic roughness factors and texture
            material = material.metallic_factor(pbr.metallic_factor());
            material = material.roughness_factor(pbr.roughness_factor());
            if let Some(gtexture) = pbr.metallic_roughness_texture() {
                material = material.metallic_roughness(Handle::from(gtexture.texture().index()));
            }

            self.materials.push(material.build());
        }

        Ok(())
    }

    fn load_vertices(
        &self,
        uri_buffers: &UriBuffers,
        gprimitive: &gltf::Primitive,
    ) -> Result<Vec<Vertex>, Box<dyn Error>> {
        let mut vertices = vec![];

        let mode = gprimitive.mode();
        assert!(mode == gltf::mesh::Mode::Triangles);

        // Load normals first, so we can process tangents later
        for (semantic, accessor) in gprimitive.attributes() {
            if semantic == gltf::mesh::Semantic::Normals {
                uri_buffers.load_normals(&mut vertices, &accessor)?;
            }
        }

        for (semantic, accessor) in gprimitive.attributes() {
            match semantic {
                gltf::mesh::Semantic::Positions => {
                    uri_buffers.load_positions(&mut vertices, &accessor)?
                }
                gltf::mesh::Semantic::TexCoords(_) => {
                    uri_buffers.load_uvs(&mut vertices, &accessor)?
                }
                gltf::mesh::Semantic::Colors(_) => {
                    uri_buffers.load_colors(&mut vertices, &accessor)?
                }
                gltf::mesh::Semantic::Normals => (), // Already loaded
                gltf::mesh::Semantic::Tangents => {
                    uri_buffers.load_tangents(&mut vertices, &accessor)
                }
                _ => println!("Skipping semantic: {:?}", semantic),
            }
        }

        Ok(vertices)
    }

    fn load_primitive(
        &mut self,
        uri_buffers: &UriBuffers,
        gprimitive: &gltf::Primitive,
    ) -> Result<Handle<Primitive>, Box<dyn Error>> {
        let vertices = self.load_vertices(uri_buffers, gprimitive)?;
        let (indices, index_size) = uri_buffers.load_indices(gprimitive);

        let material = if let Some(index) = gprimitive.material().index() {
            index as usize
        } else {
            usize::MAX
        };

        let primitive = Primitive::builder()
            .vertices(vertices)
            .indices(
                PrimitiveIndices::builder()
                    .indices(indices)
                    .index_type(index_size.into())
                    .build(),
            )
            .material(material.into())
            .build();

        Ok(self.primitives.push(primitive))
    }

    fn load_meshes(
        &mut self,
        gltf: &gltf::Gltf,
        uri_buffers: &UriBuffers,
    ) -> Result<(), Box<dyn Error>> {
        for gmesh in gltf.meshes() {
            let primitive_handles = gmesh
                .primitives()
                .map(|gprimitive| {
                    self.load_primitive(uri_buffers, &gprimitive)
                        .expect("Failed to load a primitive")
                })
                .collect();

            let mesh = Mesh::builder().primitives(primitive_handles).build();
            self.meshes.push(mesh);
        }
        Ok(())
    }

    fn load_cameras(&mut self, gltf: &gltf::Gltf) {
        for gcamera in gltf.cameras() {
            let camera = match gcamera.projection() {
                gltf::camera::Projection::Perspective(p) => {
                    let aspect_ratio = p.aspect_ratio().unwrap_or(1.0);
                    let yfov = p.yfov();
                    let near = p.znear();
                    if let Some(far) = p.zfar() {
                        Camera::finite_perspective(aspect_ratio, yfov, near, far)
                    } else {
                        Camera::infinite_perspective(aspect_ratio, yfov, near)
                    }
                }
                gltf::camera::Projection::Orthographic(o) => {
                    let width = o.xmag();
                    let height = o.ymag();
                    let near = o.znear();
                    let far = o.zfar();
                    Camera::orthographic(width, height, near, far)
                }
            };
            self.cameras.push(camera);
        }
    }

    fn create_node(gnode: &gltf::Node) -> Node {
        let transform = gnode.transform().decomposed();

        let translation = &transform.0;
        let translation = Vec3::new(translation[0], translation[1], translation[2]);

        let rotation = &transform.1;
        let rotation = Quat::new(rotation[0], rotation[1], rotation[2], rotation[3]);

        let scale = &transform.2;
        let scale = Vec3::new(scale[0], scale[1], scale[2]);

        let mut node_builder = Node::builder()
            .name(gnode.name().unwrap_or("Unknown"))
            .children(
                gnode
                    .children()
                    .map(|gchild| Handle::from(gchild.index()))
                    .collect(),
            )
            .trs(
                Trs::builder()
                    .translation(translation)
                    .rotation(rotation)
                    .scale(scale)
                    .build(),
            );

        if let Some(mesh) = gnode.mesh() {
            node_builder = node_builder.mesh(Handle::from(mesh.index()));
        }

        if let Some(camera) = gnode.camera() {
            node_builder = node_builder.camera(Handle::from(camera.index()));
        }

        node_builder.build()
    }

    fn load_nodes(&mut self, gltf: &gltf::Gltf) {
        // Load scene
        let gscene = gltf.scenes().next().unwrap();
        self.scene.children = gscene
            .nodes()
            .into_iter()
            .map(|n| n.index().into())
            .collect();

        // Load nodes
        for gnode in gltf.nodes() {
            let node = Self::create_node(&gnode);
            self.nodes.push(node);
        }
    }

    /// Returns the last node with a camera
    pub fn get_mut_last_camera(&mut self) -> Option<&mut Node> {
        self.nodes
            .iter_mut()
            .rev()
            .find(|node| node.camera.is_valid())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn load_gltf() {
        let assets = Assets::new();
        assert!(Model::load_gltf_path("test", &assets).is_err());

        let path = "tests/model/box/box.gltf";
        if let Err(err) = Model::load_gltf_path(path, &assets) {
            panic!("Failed to load \"{}\": {}", path, err);
        }
    }

    #[test]
    fn load_images() {
        let assets = Assets::new();
        let model = Model::load_gltf_path("tests/model/suzanne/suzanne.gltf", &assets).unwrap();
        assert!(model.images.len() == 2);
        assert!(!model.scene.children.is_empty());
    }

    #[test]
    fn load_phong() {
        let assets = Assets::new();
        let model = Model::load_gltf_path("tests/model/box/box-phong.gltf", &assets).unwrap();
        assert!(model.materials.len() == 1);
    }
}
