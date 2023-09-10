use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::BindGroup},
};

#[derive(Resource, Clone, Deref, ExtractResource, Reflect)]
pub struct RenderImage {
    pub image: Handle<Image>,
}

#[derive(Resource)]
struct RenderImageBindGroup(BindGroup);

enum ComputeShaderState {
    Loading,
    Init,
    Update,
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {}
}
