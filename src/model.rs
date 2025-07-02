use crate::*;

#[derive(Default)]
// Model representation based on glTF spec
pub struct Model {
    pub scene: Vec<Handle<Node>>,
    pub nodes: Pack<Node>,
    pub meshes: Pack<Mesh>,
    pub primitives: Pack<Primitive>,
    pub materials: Pack<Material>,
}
