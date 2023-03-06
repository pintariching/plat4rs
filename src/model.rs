use std::ops::Range;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, Buffer, BufferAddress, IndexFormat, RenderPass, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexStepMode,
};

use crate::Vertex;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    // pub tex_coords: [f32; 2],
    pub color: [f32; 3],
    //    pub normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> VertexBufferLayout<'a> {
        use std::mem;

        VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Material {
    pub name: String,
    //pub diffuse_texture: texture::Texture,
    pub bind_group: BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_indices: u32,
    //pub material: usize,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    // pub materials: Vec<Material>,
}

pub trait DrawModel<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        // material: &'a Material,
        camera_bind_group: &'a BindGroup,
    );

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        // material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a BindGroup,
    );

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a BindGroup);

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a BindGroup,
    );
}

impl<'a, 'b> DrawModel<'b> for RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        // material: &'a Material,
        camera_bind_group: &'a BindGroup,
    ) {
        // self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
        self.draw_mesh_instanced(mesh, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        //material: &'a Material,
        instances: Range<u32>,
        camera_bind_group: &'a BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint32);
        //self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(0, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices, 0, instances);
    }

    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a BindGroup) {
        self.draw_model_instanced(model, 0..1, camera_bind_group);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: Range<u32>,
        camera_bind_group: &'a BindGroup,
    ) {
        for mesh in &model.meshes {
            // let material = &model.materials[mesh.material];
            // self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
            self.draw_mesh_instanced(mesh, instances.clone(), camera_bind_group);
        }
    }
}
