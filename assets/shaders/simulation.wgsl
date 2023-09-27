struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    acceleration: vec3<f32>,
    index: f32
}

@group(0) @binding(0) 
var<storage, read_write> particles: array<Particle, 64>;

@group(0) @binding(1)
var weights: texture_2d<f32>;


@compute @workgroup_size(8, 8, 1)
fn init(@builtin(local_invocation_index) invocation_id: u32, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    particles[invocation_id].position += vec3<f32>(0.);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(local_invocation_index) invocation_id: u32, @builtin(num_workgroups) num_workgroups: vec3<u32>) {

    particles[invocation_id].position += vec3<f32>(1.);
}