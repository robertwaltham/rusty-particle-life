use bevy::{
    prelude::{Deref, Handle, Image, Resource},
    reflect::Reflect,
    render::{extract_resource::ExtractResource, render_resource::ShaderType},
};
use bytemuck::{Pod, Zeroable};
use rand::prelude::*;

pub const MAX_FLAVOURS: usize = 10;
pub const MAX_PARTICLES: usize = 64;

#[derive(ShaderType, Pod, Zeroable, Clone, Copy, Resource, Reflect, ExtractResource, Debug)]
#[repr(C)]
pub struct Particle {
    position: [f32; 3],
    _padding1: f32, // https://stackoverflow.com/a/75525055
    velocity: [f32; 3],
    _padding2: f32,
    acceleration: [f32; 3], // TODO: is this needed
    index: f32,
}

impl Default for Particle {
    fn default() -> Self {
        let p: [f32; 3] = [random::<f32>() * 512., random::<f32>() * 512., 0.];

        Self {
            position: p,
            _padding1: Default::default(),
            velocity: random(),
            _padding2: Default::default(),
            acceleration: Default::default(),
            index: Default::default(),
        }
    }
}

// #[derive(Resource, Reflect, ExtractResource, Clone, Copy, Default, Pod, Zeroable)]
// #[repr(C)]
// pub struct Weights([[f32; MAX_FLAVOURS]; MAX_FLAVOURS]);

#[derive(Resource, Reflect, ExtractResource, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Particles([Particle; MAX_PARTICLES]);
impl Default for Particles {
    fn default() -> Self {
        Self([Particle::default(); MAX_PARTICLES])
    }
}

#[derive(Resource, Reflect, ExtractResource, Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
pub struct ParticleColours([[f32; 4]; MAX_FLAVOURS]);

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct RenderImage {
    pub image: Handle<Image>,
}

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct WeightsImage {
    pub image: Handle<Image>,
}
