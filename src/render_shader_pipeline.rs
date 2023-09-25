use std::borrow::Cow;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_graph::{self},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            CachedComputePipelineId, CachedPipelineState, ComputePassDescriptor,
            ComputePipelineDescriptor, PipelineCache, ShaderStages, StorageTextureAccess,
            TextureFormat, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
    },
};

use crate::{
    render::{ComputeShaderState, RenderImage},
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
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::WriteOnly,
                            format: TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }],
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
) {
    let output_view = &gpu_images[&output_image.image];

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("render bind group"),
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&output_view.texture_view),
        }],
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
