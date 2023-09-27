struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    acceleration: vec3<f32>,
    index: f32
}

@group(0) @binding(0) 
var<storage, read_write> particles: array<Particle, 64>;

// @group(0) @binding(1)
// var weights: texture_2d<f32>;


@compute @workgroup_size(8, 8, 1)
fn init(@builtin(local_invocation_index) invocation_id: u32, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    particles[invocation_id].position += vec3<f32>(256.);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(local_invocation_index) invocation_id: u32, @builtin(num_workgroups) num_workgroups: vec3<u32>) {

    var position = particles[invocation_id].position;
    var velocity = particles[invocation_id].velocity;

    let max = 512.;

    position += velocity;

    if position.x < 0. {
        position.x = max;
    } else if position.x > max {
        position.x = 0.;
    }

    if position.y < 0. {
        position.y = max;
    } else if position.y > max {
        position.y = 0.;
    }

    particles[invocation_id].position = position;
}