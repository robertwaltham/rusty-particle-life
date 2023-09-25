struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    acceleration: vec3<f32>,
    index: f32
}

@group(0) @binding(0) 
var<storage> particles: array<Particle, 16>;


@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
}