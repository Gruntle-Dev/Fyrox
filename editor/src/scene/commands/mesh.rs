use crate::{command::Command, scene::commands::SceneContext};
use fyrox::{
    core::{pool::Handle, sstorage::ImmutableString},
    material::{shader::SamplerFallback, PropertyValue},
    resource::texture::Texture,
    scene::{mesh::Mesh, node::Node},
};

#[derive(Debug)]
enum TextureSet {
    Single(Texture),
    Multiple(Vec<Option<Texture>>),
}

#[derive(Debug)]
pub struct SetMeshTextureCommand {
    node: Handle<Node>,
    set: TextureSet,
}

impl SetMeshTextureCommand {
    pub fn new(node: Handle<Node>, texture: Texture) -> Self {
        Self {
            node,
            set: TextureSet::Single(texture),
        }
    }
}

impl Command for SetMeshTextureCommand {
    fn name(&mut self, _context: &SceneContext) -> String {
        "Set Texture".to_owned()
    }

    fn execute(&mut self, context: &mut SceneContext) {
        if let TextureSet::Single(texture) = &self.set {
            let mesh: &mut Mesh = context.scene.graph[self.node].as_mesh_mut();
            let old_set = mesh
                .surfaces_mut()
                .iter()
                .map(|s| {
                    s.material()
                        .lock()
                        .property_ref(&ImmutableString::new("diffuseTexture"))
                        .and_then(|p| {
                            if let PropertyValue::Sampler { value, .. } = p {
                                value.clone()
                            } else {
                                None
                            }
                        })
                })
                .collect();
            for surface in mesh.surfaces_mut() {
                surface
                    .material()
                    .lock()
                    .set_property(
                        &ImmutableString::new("diffuseTexture"),
                        PropertyValue::Sampler {
                            value: Some(texture.clone()),
                            fallback: SamplerFallback::White,
                        },
                    )
                    .unwrap();
            }
            self.set = TextureSet::Multiple(old_set);
        } else {
            unreachable!()
        }
    }

    fn revert(&mut self, context: &mut SceneContext) {
        if let TextureSet::Multiple(set) = &self.set {
            let mesh: &mut Mesh = context.scene.graph[self.node].as_mesh_mut();
            let new_value = mesh.surfaces_mut()[0]
                .material()
                .lock()
                .property_ref(&ImmutableString::new("diffuseTexture"))
                .and_then(|p| {
                    if let PropertyValue::Sampler { value, .. } = p {
                        value.clone()
                    } else {
                        None
                    }
                })
                .unwrap();
            assert_eq!(mesh.surfaces_mut().len(), set.len());
            for (surface, old_texture) in mesh.surfaces_mut().iter_mut().zip(set) {
                surface
                    .material()
                    .lock()
                    .set_property(
                        &ImmutableString::new("diffuseTexture"),
                        PropertyValue::Sampler {
                            value: old_texture.clone(),
                            fallback: SamplerFallback::White,
                        },
                    )
                    .unwrap();
            }
            self.set = TextureSet::Single(new_value);
        } else {
            unreachable!()
        }
    }
}
