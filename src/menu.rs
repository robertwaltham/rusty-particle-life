use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Button, FontId, RichText},
    EguiContexts, EguiPlugin,
};
const PANEL_WIDTH: f32 = 200.;
pub struct Menu;
impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, ui_system);
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    // mut next_state: ResMut<NextState<AppState>>,
    // state: Res<State<AppState>>,
    // type_registry: Res<AppTypeRegistry>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .resizable(false)
        .min_width(PANEL_WIDTH)
        .show(ctx, |ui| {});
}
