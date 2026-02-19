use std::{collections::BTreeMap, sync::Arc};

use super::*;
use depkg::pkg_parser::tex_parser::Tex;
use wgpu::*;

pub struct WgpuApp {
    surface: AppSurface,

    buffers: buffer::Buffers,

    bindgroups: BindGroups,

    scene: crate::scene::loader::scene::Root,

    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,

    audio: rodio::OutputStream,
}

pub struct AppSurface {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    size: [u32; 2],
}

pub struct BindGroups {
    texture_layout: BindGroupLayout,
    projection_layout: BindGroupLayout,

    texture: Option<BindGroup>,
    projection: Option<BindGroup>,
}

pub struct Scene {
    root: crate::scene::loader::scene::Root,
    textures: Arc<BTreeMap<String, Arc<Tex>>>,
    jsons: Arc<BTreeMap<String, String>>,
    desc: Arc<BTreeMap<String, Arc<Vec<u8>>>>,
}
