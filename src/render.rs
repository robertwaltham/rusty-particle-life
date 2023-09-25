use bevy::{
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_graph::RenderGraph,
        Render, RenderApp, RenderSet,
    },
};

use crate::render_shader_pipeline::{RenderShaderNode, RenderShaderPipeline};
use crate::sim_shader_pipeline::{SimulationShaderNode, SimulationShaderPipeline};

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct RenderImage {
    pub image: Handle<Image>,
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
        app.add_plugins((ExtractResourcePlugin::<RenderImage>::default(),));

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_systems(
            Render,
            (
                crate::sim_shader_pipeline::queue_bind_group,
                crate::render_shader_pipeline::queue_bind_group,
            )
                .in_set(RenderSet::Queue),
        );

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
