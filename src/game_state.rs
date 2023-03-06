use glam::Vec2;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device,
    ShaderStages,
};
use winit::dpi::PhysicalSize;

use crate::{
    camera::{Camera, CameraController, CameraUniform},
    model::{Mesh, Model, ModelVertex},
};

pub struct GameState {
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub camera_bind_group_layout: BindGroupLayout,
    pub model: Model,
}

impl GameState {
    pub fn new(device: &Device, window_size: &PhysicalSize<u32>) -> Self {
        let camera = Camera {
            focus_position: Vec2::new(100., 0.),
            zoom: 2.,
        };

        let camera_controller = CameraController { speed: 1. };

        let camera_uniform = CameraUniform::new(&camera, window_size);
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let vertices = &[
            ModelVertex {
                position: [0., 0., 0.],
                color: [1., 0., 0.],
            },
            ModelVertex {
                position: [100., 0., 0.],
                color: [0., 1., 0.],
            },
            ModelVertex {
                position: [100., 100., 0.],
                color: [0., 0., 1.],
            },
        ];

        let indices = &[0, 2, 1];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        let mesh = Mesh {
            name: "Mesh".into(),
            vertex_buffer,
            index_buffer,
            num_indices: 3,
        };

        let model = Model { meshes: vec![mesh] };

        Self {
            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
            model,
        }
    }
}
