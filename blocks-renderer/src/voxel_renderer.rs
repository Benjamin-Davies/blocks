use std::mem;

use glam::{u8vec3, I8Vec3, U8Vec3};
use wgpu::util::DeviceExt;

use blocks_game::{block::Block, subchunk::Subchunk};

use crate::texture;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Vertex {
    position: U8Vec3,
    block_type: u8,
    normal: I8Vec3,
    _padding: u8,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Uint8x4, 1 => Sint8x4];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct VoxelRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
}

impl VoxelRenderer {
    pub fn new(
        device: &wgpu::Device,
        render_pipeline_layout: &wgpu::PipelineLayout,
        color_target_format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("voxel_shader.wgsl"));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Voxel Render Pipeline"),
            layout: Some(render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: color_target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
        }
    }

    pub fn update_subchunk(&mut self, device: &wgpu::Device, subchunk: &Subchunk) {
        let mut vertices = Vec::new();
        for x in 0..Subchunk::SIZE {
            for y in 0..Subchunk::SIZE {
                for z in 0..Subchunk::SIZE {
                    faces_for_block(&mut vertices, subchunk, x, y, z);
                }
            }
        }

        let mut indices = Vec::new();
        for i in (0..vertices.len() as u16).step_by(4) {
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2);
            indices.push(i);
            indices.push(i + 2);
            indices.push(i + 3);
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Voxel Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Voxel Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.num_indices = num_indices;
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass, camera_bind_group: &wgpu::BindGroup) {
        let (Some(vertex_buffer), Some(index_buffer)) = (&self.vertex_buffer, &self.index_buffer)
        else {
            return;
        };

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

fn faces_for_block(vertices: &mut Vec<Vertex>, subchunk: &Subchunk, x: usize, y: usize, z: usize) {
    let (x, y, z) = (x as isize, y as isize, z as isize);
    let block = subchunk.block_or_air(x, y, z);
    if block == Block::AIR {
        return;
    }

    let position = u8vec3(x as u8, y as u8, z as u8);
    let block_type = bytemuck::cast(block);

    // -X
    if subchunk.block_or_air(x - 1, y, z) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(0, 0, 0),
                block_type,
                normal: I8Vec3::NEG_X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 0, 1),
                block_type,
                normal: I8Vec3::NEG_X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 1, 1),
                block_type,
                normal: I8Vec3::NEG_X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 1, 0),
                block_type,
                normal: I8Vec3::NEG_X,
                _padding: 0,
            },
        ]);
    }

    // +X
    if subchunk.block_or_air(x + 1, y, z) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(1, 0, 0),
                block_type,
                normal: I8Vec3::X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 0),
                block_type,
                normal: I8Vec3::X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 1),
                block_type,
                normal: I8Vec3::X,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 0, 1),
                block_type,
                normal: I8Vec3::X,
                _padding: 0,
            },
        ]);
    }

    // -Y
    if subchunk.block_or_air(x, y - 1, z) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(0, 0, 0),
                block_type,
                normal: I8Vec3::NEG_Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 0, 0),
                block_type,
                normal: I8Vec3::NEG_Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 0, 1),
                block_type,
                normal: I8Vec3::NEG_Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 0, 1),
                block_type,
                normal: I8Vec3::NEG_Y,
                _padding: 0,
            },
        ]);
    }

    // +Y
    if subchunk.block_or_air(x, y + 1, z) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(0, 1, 0),
                block_type,
                normal: I8Vec3::Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 1, 1),
                block_type,
                normal: I8Vec3::Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 1),
                block_type,
                normal: I8Vec3::Y,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 0),
                block_type,
                normal: I8Vec3::Y,
                _padding: 0,
            },
        ]);
    }

    // -Z
    if subchunk.block_or_air(x, y, z - 1) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(0, 0, 0),
                block_type,
                normal: I8Vec3::NEG_Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 1, 0),
                block_type,
                normal: I8Vec3::NEG_Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 0),
                block_type,
                normal: I8Vec3::NEG_Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 0, 0),
                block_type,
                normal: I8Vec3::NEG_Z,
                _padding: 0,
            },
        ]);
    }

    // +Z
    if subchunk.block_or_air(x, y, z + 1) == Block::AIR {
        vertices.extend([
            Vertex {
                position: position + u8vec3(0, 0, 1),
                block_type,
                normal: I8Vec3::Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 0, 1),
                block_type,
                normal: I8Vec3::Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(1, 1, 1),
                block_type,
                normal: I8Vec3::Z,
                _padding: 0,
            },
            Vertex {
                position: position + u8vec3(0, 1, 1),
                block_type,
                normal: I8Vec3::Z,
                _padding: 0,
            },
        ]);
    }
}
