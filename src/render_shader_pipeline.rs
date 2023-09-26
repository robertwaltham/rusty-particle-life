use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{self},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            BufferBindingType, BufferSize, CachedComputePipelineId, CachedPipelineState,
            ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderStages,
            StorageTextureAccess, TextureFormat, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
    },
};

use crate::{
    objects::{ParticleColours, Particles, RenderImage},
    render::{ComputeShaderState, ParticleBuffer, ParticleColourBuffer},
    SIZE, WORKGROUP_SIZE,
};

#[derive(Resource)]
pub struct RenderBindGroup(pub BindGroup);

#[derive(Resource)]
pub struct RenderShaderPipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for RenderShaderPipeline {
    fn from_world(world: &mut World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("render bind group"),
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::WriteOnly,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
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
                        // BindGroupLayoutEntry {
                        //     binding: 1,
                        //     visibility: ShaderStages::COMPUTE,
                        //     ty: BindingType::Buffer {
                        //         ty: BufferBindingType::Uniform,
                        //         has_dynamic_offset: false,
                        //         min_binding_size: BufferSize::new(
                        //             std::mem::size_of::<Weights>() as u64
                        //         ),
                        //     },
                        //     count: None,
                        // },
                        BindGroupLayoutEntry {
                            binding: 2,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    std::mem::size_of::<ParticleColours>() as u64,
                                ),
                            },
                            count: None,
                        },
                    ],
                });
        let shader = world.resource::<AssetServer>().load("shaders/render.wgsl");
        let pipeline_cache = world.resource_mut::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("render init pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            push_constant_ranges: vec![],
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("render update pipeline")),
            layout: vec![texture_bind_group_layout.clone()],
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            push_constant_ranges: vec![],
        });

        RenderShaderPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<RenderShaderPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    output_image: Res<RenderImage>,
    render_device: Res<RenderDevice>,
    particles_buffer: Res<ParticleBuffer>,
    // weights_buffer: Res<WeightsBuffer>,
    particle_colours_buffer: Res<ParticleColourBuffer>,
) {
    let output_view: &bevy::render::texture::GpuImage = &gpu_images[&output_image.image];

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("render bind group"),
        layout: &pipeline.texture_bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&output_view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: particles_buffer
                    .buffer
                    .as_ref()
                    .unwrap()
                    .as_entire_binding(),
            },
            // BindGroupEntry {
            //     binding: 1,
            //     resource: weights_buffer.buffer.as_ref().unwrap().as_entire_binding(),
            // },
            BindGroupEntry {
                binding: 2,
                resource: particle_colours_buffer
                    .buffer
                    .as_ref()
                    .unwrap()
                    .as_entire_binding(),
            },
        ],
    });
    commands.insert_resource(RenderBindGroup(bind_group));
}

pub struct RenderShaderNode {
    pub state: ComputeShaderState,
}

impl Default for RenderShaderNode {
    fn default() -> Self {
        Self {
            state: ComputeShaderState::Loading,
        }
    }
}

impl render_graph::Node for RenderShaderNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<RenderShaderPipeline>();
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
        let texture_bind_group = &world.resource::<RenderBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<RenderShaderPipeline>();

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
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE.0, SIZE.1 / WORKGROUP_SIZE.1, 1);
            }
            ComputeShaderState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE.0, SIZE.1 / WORKGROUP_SIZE.1, 1);
            }
        }

        Ok(())
    }
}
