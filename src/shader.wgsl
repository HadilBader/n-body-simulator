const GRAVITATIONAL_CONSTANT: f32 = 1;

struct Body {
    mass: f32,
    position: vec2<f32>,
    velocity: vec2<f32>,
};

fn update_position(body: ptr<function, Body>, other_body: ptr<function, Body>, dt: f32) {
    let r = body.position - other_body.position;

    let norm_r = length(r);
    let mu = GRAVITATIONAL_CONSTANT * other_body.mass;
    let acceleration = -r * (mu / pow(norm_r, 3.0));

    body.velocity += acceleration * dt;
    position += body.velocity * dt;
}

@group(0) @binding(0) var<storage, read_write> bodies: array<Body>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i >= arrayLength(&bodies)) { return; }

    let body = bodies[i];

    for (var j = 0u; j < arrayLength(&bodies); j = j + 1u) {
        if (i == j) { continue; }

        let other_body = bodies[j];
        update_position(&body, &other_body, 0.00001);
    }
}