use bevy::{
    prelude::{Deref, Handle, Image, Resource},
    reflect::Reflect,
    render::{extract_resource::ExtractResource, render_resource::ShaderType},
};
use bytemuck::{Pod, Zeroable};
use rand::prelude::*;

use crate::SIZE;

pub const MAX_FLAVOURS: usize = 10;
pub const MAX_PARTICLES: usize = 64;

#[derive(
    ShaderType, Pod, Zeroable, Clone, Copy, Resource, Reflect, ExtractResource, Debug, Default,
)]
#[repr(C)]
pub struct Particle {
    position: [f32; 3],
    _padding1: f32, // https://stackoverflow.com/a/75525055
    velocity: [f32; 3],
    _padding2: f32,
    acceleration: [f32; 3], // TODO: is this needed
    index: f32,
}

// #[derive(Resource, Reflect, ExtractResource, Clone, Copy, Default, Pod, Zeroable)]
// #[repr(C)]
// pub struct Weights([[f32; MAX_FLAVOURS]; MAX_FLAVOURS]);

#[derive(Resource, Reflect, ExtractResource, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct Particles([Particle; MAX_PARTICLES]);
impl Default for Particles {
    fn default() -> Self {
        let mut particles = [Particle::default(); MAX_PARTICLES];

        for i in 0..MAX_PARTICLES {
            particles[i].position = [
                (random::<f32>() * (SIZE.0 as f32)) - (SIZE.0 as f32 / 2.),
                (random::<f32>() * (SIZE.1 as f32)) - (SIZE.1 as f32 / 2.),
                0.,
            ];

            particles[i].velocity = [random::<f32>() - 0.5, random::<f32>() - 0.5, 0.]
        }

        println!("{:?}", particles);
        Self(particles)
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
