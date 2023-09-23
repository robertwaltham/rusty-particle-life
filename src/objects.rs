use bevy::{
    prelude::Resource,
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
struct Particle {
    position: [f32; 3],
    _padding1: f32,
    velocity: [f32; 3],
    _padding2: f32,
    acceleration: [f32; 3], // TODO: is this needed
    _padding3: f32,
    index: f32,
}

#[derive(Resource)]
#[repr(C)] // TODO: is this needed for the struct, since only the underlying array will be passed into gpu land
struct Weights([[f32; MAX_FLAVOURS]; MAX_FLAVOURS]);

#[derive(Resource)]
#[repr(C)]
struct Particles([Particle; MAX_PARTICLES]);
