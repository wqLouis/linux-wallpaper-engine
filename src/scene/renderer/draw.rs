use std::{collections::BTreeMap, path::Path, rc::Rc};

use super::buffer::Buffers;
use bytemuck::bytes_of;
use depkg::pkg_parser::tex_parser::Tex;
use glam::{Mat2, Vec2, Vec3};
use serde_json::{Map, Value, from_value};
use wgpu::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
    texture_index: u32,
}

pub struct DrawTextureObject {
    pub texture: Rc<Tex>,
    origin: Vec3,
    angles: Vec3,
    scale: Vec3,
    size: Vec2,
    alpha: f32,
}

pub struct DrawQueue {
    pub queue: Vec<DrawTextureObject>,
}

impl DrawQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push(&mut self, draw_texture_object: DrawTextureObject) {
        self.queue.push(draw_texture_object);
    }

    pub fn submit_draw_queue(self, buffers: &mut Buffers, queue: &Queue) {
        for (index, draw_obj) in self.queue.into_iter().enumerate() {
            draw_obj.draw(buffers, queue, index as u32);
        }
    }
}

impl DrawTextureObject {
    pub fn new(
        object: &crate::scene::loader::scene::Object,
        jsons: &BTreeMap<String, String>,
        textures: &BTreeMap<String, Rc<Tex>>,
    ) -> Option<Self> {
        let visible =
            from_value::<bool>((&object.visible.clone().unwrap_or(Value::Bool(true))).to_owned());
        let visible_object = from_value::<bool>(
            (&object
                .visible
                .clone()
                .unwrap_or(Value::Bool(true))
                .as_object()
                .unwrap_or(&Map::default())
                .get("value")
                .unwrap_or(&Value::Bool(true))
                .to_owned())
                .to_owned(),
        );

        if !visible.unwrap_or(true) | !visible_object.unwrap_or(true) {
            return None;
        }

        let origin = &object.origin.clone()?.parse()?;
        let angles = &object.angles.clone().unwrap_or_default().parse()?;
        let scale = &object.scale.clone().unwrap_or_default().parse()?;
        let size = &object.size.clone()?.parse()?;
        let alpha = 1.0;
        let image = &object.image.clone()?;
        let mut model = Path::new(jsons.get(image)?).to_path_buf();
        model.set_extension("tex");
        let texture = Rc::clone(textures.get(model.to_str()?)?);

        Some(Self {
            texture,
            origin: Vec3 {
                x: origin[0],
                y: origin[1],
                z: origin[2],
            },
            angles: Vec3 {
                x: angles[0],
                y: angles[1],
                z: angles[2],
            },
            scale: Vec3 {
                x: scale[0],
                y: scale[1],
                z: scale[2],
            },
            size: Vec2 {
                x: size[0],
                y: size[1],
            },
            alpha,
        })
    }

    fn draw(self, buffers: &mut Buffers, queue: &Queue, texture_index: u32) {
        // consume itself and write the data into buffers

        let rotation_mat = Mat2::from_angle(self.angles.z.to_radians());
        let rotated = vec![
            Vec2::new(-self.size.x / 2.0, self.size.y / 2.0),
            Vec2::new(self.size.x / 2.0, self.size.y / 2.0),
            Vec2::new(self.size.x / 2.0, -self.size.y / 2.0),
            Vec2::new(-self.size.x / 2.0, -self.size.y / 2.0),
        ]
        .iter()
        .map(|vertex| {
            (rotation_mat * vertex)
                + Vec2::new(
                    self.origin.x + self.size.x / 2.0,
                    self.origin.y + self.size.y / 2.0,
                )
        })
        .collect::<Vec<Vec2>>();

        let rect = [
            Vertex {
                pos: [rotated[0].x, rotated[0].y, self.origin.z],
                uv: [0.0, 0.0],
                texture_index,
            },
            Vertex {
                pos: [rotated[1].x, rotated[1].y, self.origin.z],
                uv: [1.0, 0.0],
                texture_index,
            },
            Vertex {
                pos: [rotated[2].x, rotated[2].y, self.origin.z],
                uv: [1.0, 1.0],
                texture_index,
            },
            Vertex {
                pos: [rotated[3].x, rotated[3].y, self.origin.z],
                uv: [0.0, 1.0],
                texture_index,
            },
        ];

        let indices: [u16; 6] = [0, 2, 1, 0, 3, 2].map(|f| f + buffers.index_len as u16);

        queue.write_buffer(
            &buffers.vertex,
            std::mem::size_of::<Vertex>() as u64 * buffers.vertex_len as u64,
            bytes_of(&rect),
        );

        queue.write_buffer(
            &buffers.index,
            std::mem::size_of::<[u16; 6]>() as u64 * buffers.index_len as u64,
            bytes_of(&indices),
        );

        buffers.index_len += indices.len() as u32;
        buffers.vertex_len += rect.len() as u32;
    }
}

impl Vertex {
    pub fn create_buffer_layout<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as BufferAddress,
                    shader_location: 2,
                    format: VertexFormat::Uint32,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as BufferAddress,
                    shader_location: 3,
                    format: VertexFormat::Float32,
                },
            ],
        }
    }
}
