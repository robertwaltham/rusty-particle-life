use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::RenderGraph,
        render_resource::{Buffer, BufferDescriptor, BufferUsages},
        renderer::{RenderDevice, RenderQueue},
        Render, RenderApp, RenderSet,
    },
};
use bytemuck::bytes_of;

use crate::{
    objects::{ParticleColours, Particles, Weights},
    render_shader_pipeline::{RenderShaderNode, RenderShaderPipeline},
    sim_shader_pipeline::{SimulationShaderNode, SimulationShaderPipeline},
};

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct RenderImage {
    pub image: Handle<Image>,
}

#[derive(Resource, Debug)]
pub struct ParticleBuffer {
    pub buffer: Option<Buffer>,
}

#[derive(Resource, Debug)]
pub struct WeightsBuffer {
    pub buffer: Option<Buffer>,
}

#[derive(Resource, Debug)]
pub struct ParticleColourBuffer {
    pub buffer: Option<Buffer>,
}

pub enum ComputeShaderState {
    Loading,
    Init,
    Update,
}

pub struct RenderPlugin;

const SIMULATION: &str = "simulation";
const RENDER: &str = "render";

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractResourcePlugin::<RenderImage>::default(),
            ExtractResourcePlugin::<Weights>::default(),
            ExtractResourcePlugin::<Particles>::default(),
            ExtractResourcePlugin::<ParticleColours>::default(),
        ));

        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .add_systems(
                Render,
                (
                    crate::sim_shader_pipeline::queue_bind_group,
                    crate::render_shader_pipeline::queue_bind_group,
                )
                    .in_set(RenderSet::Queue),
            )
            .add_systems(Render, prepare_buffers.in_set(RenderSet::Prepare))
            .insert_resource(ParticleBuffer { buffer: None })
            .insert_resource(WeightsBuffer { buffer: None })
            .insert_resource(ParticleColourBuffer { buffer: None });

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(SIMULATION, SimulationShaderNode::default());
        render_graph.add_node(RENDER, RenderShaderNode::default());

        render_graph.add_node_edge(SIMULATION, bevy::render::main_graph::node::CAMERA_DRIVER);
        render_graph.add_node_edge(RENDER, SIMULATION);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<SimulationShaderPipeline>()
            .init_resource::<RenderShaderPipeline>();
    }
}

fn prepare_buffers(
    particles: Res<Particles>,
    weights: Res<Weights>,
    particle_colours: Res<ParticleColours>,
    mut particles_buffer: ResMut<ParticleBuffer>,
    mut weights_buffer: ResMut<WeightsBuffer>,
    mut particle_colours_buffer: ResMut<ParticleColourBuffer>,
    render_queue: Res<RenderQueue>,
    render_device: Res<RenderDevice>,
) {
    if particles_buffer.buffer.is_none() {
        particles_buffer.buffer = Some(render_device.create_buffer(&BufferDescriptor {
            label: Some("particles buffer"),
            size: std::mem::size_of::<Particles>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
    }

    if weights_buffer.buffer.is_none() {
        weights_buffer.buffer = Some(render_device.create_buffer(&BufferDescriptor {
            label: Some("weights buffer"),
            size: std::mem::size_of::<Weights>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
    }

    if particle_colours_buffer.buffer.is_none() {
        particle_colours_buffer.buffer = Some(render_device.create_buffer(&BufferDescriptor {
            label: Some("spheres buffer"),
            size: std::mem::size_of::<ParticleColours>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
    }

    render_queue.write_buffer(
        &particles_buffer.buffer.as_ref().unwrap(),
        0,
        bytes_of(particles.as_ref()),
    );

    render_queue.write_buffer(
        &weights_buffer.buffer.as_ref().unwrap(),
        0,
        bytes_of(weights.as_ref()),
    );

    render_queue.write_buffer(
        &particle_colours_buffer.buffer.as_ref().unwrap(),
        0,
        bytes_of(particle_colours.as_ref()),
    );
}
