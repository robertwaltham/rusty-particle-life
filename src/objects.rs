use bevy::{
    prelude::{Color, Resource},
    reflect::Reflect,
    render::{extract_resource::ExtractResource, render_resource::ShaderType},
};
use bytemuck::{Pod, Zeroable};

const MAX_FLAVOURS: usize = 10;
const MAX_PARTICLES: usize = 1024;

#[derive(
    ShaderType, Pod, Zeroable, Clone, Copy, Resource, Reflect, ExtractResource, Default, Debug,
)]
#[repr(C)]
pub struct Particle {
    position: [f32; 3],
    _padding1: f32, // https://stackoverflow.com/a/75525055
    velocity: [f32; 3],
    _padding2: f32,
    acceleration: [f32; 3], // TODO: is this needed
    _padding3: f32,
    index: f32,
}

#[derive(Resource, Reflect, ExtractResource, Clone, Copy, Default)]
#[repr(C)] // TODO: is this needed for the struct, since only the underlying array will be passed into gpu land
pub struct Weights([[f32; MAX_FLAVOURS]; MAX_FLAVOURS]);

#[derive(Resource, Reflect, ExtractResource, Clone, Copy)]
#[repr(C)]
pub struct Particles([Particle; MAX_PARTICLES]);
impl Default for Particles {
    fn default() -> Self {
        Self([Particle::default(); MAX_PARTICLES])
    }
}

#[derive(Resource, Reflect, ExtractResource, Clone, Copy, Default)]
pub struct ParticleColours([Color; MAX_FLAVOURS]);
