use crate::render::RenderPlugin;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    sprite::Anchor,
    window::{PrimaryWindow, Window},
};
use menu::Menu;
use objects::*;

pub mod menu;
pub mod objects;
pub mod render;
pub mod render_shader_pipeline;
pub mod sim_shader_pipeline;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Waiting,
    Running,
    Done,
    Reset,
}

const SIZE: (u32, u32) = (512, 512);
const WORKGROUP_SIZE: (u32, u32, u32) = (8, 8, 1);

#[cfg(target_arch = "wasm32")]
fn main() {
    use js_sys::Object;
    use web_sys::console;

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // https://developer.mozilla.org/en-US/docs/Web/API/GPU
    // Firefox has navigator.gpu, despite the docs saying it's nightly only
    let gpu = web_sys::window().unwrap().navigator().gpu();
    if gpu.is_undefined()
        || !Object::get_prototype_of(&gpu).has_own_property(&"wgslLanguageFeatures".into())
    {
        let error_msg = "Sorry, WebGPU is not supported on this browser. Only Chrome 113 and later is supported.";
        console::log_1(&error_msg.into());

        let val = document.create_element("p").expect("no element");
        val.set_text_content(Some(error_msg));

        body.append_child(&val).expect("couldn't add node");
    } else {
        run();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run();
}

fn run() {
    App::new()
        .add_plugins((DefaultPlugins, Menu, RenderPlugin))
        .add_systems(Startup, setup)
        .add_state::<AppState>()
        .init_resource::<Particles>()
        .init_resource::<ParticleColours>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = window_query
        .get_single()
        .expect("Expected to have a primary window");

    let resolution = window.resolution.clone();

    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image_handle = images.add(image);

    commands
        .spawn(SpriteBundle {
            texture: image_handle.clone(),
            sprite: Sprite { ..default() },
            ..default()
        })
        .insert(Name::new("Render Sprite"));

    commands.insert_resource(RenderImage {
        image: image_handle.clone(),
    });

    let mut weights = Image::new_fill(
        Extent3d {
            width: MAX_FLAVOURS as u32,
            height: MAX_FLAVOURS as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[128, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );
    weights.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let weights_handle = images.add(weights);

    commands
        .spawn(SpriteBundle {
            texture: weights_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: resolution.width() / 2. - 10.,
                    y: -resolution.height() / 2. + 10.,
                    z: 0.,
                },
                rotation: Quat::IDENTITY,
                scale: Vec3 {
                    x: 10.,
                    y: 10.,
                    z: 1.,
                },
            },
            sprite: Sprite {
                anchor: Anchor::BottomRight,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Weights Sprite"));

    commands.insert_resource(WeightsImage {
        image: weights_handle.clone(),
    });
}
