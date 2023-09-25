use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_graph::{self},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
            BufferSize, CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor,
            ComputePipelineDescriptor, PipelineCache, ShaderStages,
        },
        renderer::{RenderContext, RenderDevice},
    },
};

use crate::{
    objects::{Particles, Weights},
    render::{ComputeShaderState, ParticleBuffer, WeightsBuffer},
};

#[derive(Resource)]
pub struct SimulationBindGroup(pub BindGroup);

#[derive(Resource)]
pub struct SimulationShaderPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for SimulationShaderPipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("sim bind group"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    std::mem::size_of::<Particles>() as u64
                                ),
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    std::mem::size_of::<Weights>() as u64
                                ),
                            },
                            count: None,
                        },
                        // BindGroupLayoutEntry {
                        //     binding: 2,
                        //     visibility: ShaderStages::COMPUTE,
                        //     ty: BindingType::Buffer {
                        //         ty: BufferBindingType::Uniform,
                        //         has_dynamic_offset: false,
                        //         min_binding_size: BufferSize::new(
                        //             std::mem::size_of::<ParticleColours>() as u64,
                        //         ),
                        //     },
                        //     count: None,
                        // },
                    ],
                });
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/simulation.wgsl");
        let pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("sim init pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            push_constant_ranges: vec![],
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("sim update pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            push_constant_ranges: vec![],
        });

        SimulationShaderPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<SimulationShaderPipeline>,
    render_device: Res<RenderDevice>,
    particles_buffer: Res<ParticleBuffer>,
    weights_buffer: Res<WeightsBuffer>,
    // particle_colours_buffer: Res<ParticleColourBuffer>,
) {
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("sim bind group"),
        layout: &pipeline.texture_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: particles_buffer
                    .buffer
                    .as_ref()
                    .unwrap()
                    .as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: weights_buffer.buffer.as_ref().unwrap().as_entire_binding(),
            },
            // BindGroupEntry {
            //     binding: 2,
            //     resource: particle_colours_buffer
            //         .buffer
            //         .as_ref()
            //         .unwrap()
            //         .as_entire_binding(),
            // },
        ],
    });
    commands.insert_resource(SimulationBindGroup(bind_group));
}

pub struct SimulationShaderNode {
    pub state: ComputeShaderState,
}

impl Default for SimulationShaderNode {
    fn default() -> Self {
        Self {
            state: ComputeShaderState::Loading,
        }
    }
}

impl render_graph::Node for SimulationShaderNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<SimulationShaderPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            ComputeShaderState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = ComputeShaderState::Init;
                }
            }
            ComputeShaderState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = ComputeShaderState::Update;
                }
            }
            ComputeShaderState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<SimulationBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<SimulationShaderPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        // select the pipeline based on the current state
        match self.state {
            ComputeShaderState::Loading => {}
            ComputeShaderState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(1, 1, 1); // TODO: workgroup size
            }
            ComputeShaderState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(1, 1, 1); // TODO: workgroup size
            }
        }

        Ok(())
    }
}
