
@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, write>;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    acceleration: vec3<f32>,
    index: f32
}

@group(0) @binding(1) 
var<storage, read_write> particles: array<Particle, 64>;


@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let color = vec4<f32>(0.5, 0.5, 0.5, 1.0);
    textureStore(texture, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(local_invocation_index) invocation_id: u32, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let particle = particles[invocation_id];
    let color = vec4<f32>(1., 0., 0., 1.0);

    let position = vec3<i32>(particle.position).xy;

    let stride = 3;

    for (var i = position.x - stride; i < position.x + stride; i++) {
        for (var j = position.y - stride; j < position.y + stride; j++) {
            textureStore(texture, vec2<i32>(i, j), color);
        }
    }
}