struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
}

struct CircleInstance {
    @location(0) center: vec2<f32>,
    @location(1) radius: f32,
    @location(2) color: vec4<f32>,
    @location(3) transform_position: vec2<f32>,
    @location(4) transform_scale: vec2<f32>,
    @location(5) transform_rotation: f32,
}

// Quad vertices for instanced rendering
const QUAD_VERTICES: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0), // Bottom-left
    vec2<f32>( 1.0, -1.0), // Bottom-right
    vec2<f32>(-1.0,  1.0), // Top-left
    vec2<f32>(-1.0,  1.0), // Top-left
    vec2<f32>( 1.0, -1.0), // Bottom-right
    vec2<f32>( 1.0,  1.0), // Top-right
);

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32, instance: CircleInstance) -> VertexOutput {
    let quad_vertex = QUAD_VERTICES[vertex_index];
    
    // Apply rotation
    let cos_rot = cos(instance.transform_rotation);
    let sin_rot = sin(instance.transform_rotation);
    let rotated_vertex = vec2<f32>(
        quad_vertex.x * cos_rot - quad_vertex.y * sin_rot,
        quad_vertex.x * sin_rot + quad_vertex.y * cos_rot
    );
    
    // Apply scale and position
    let scaled_vertex = rotated_vertex * instance.transform_scale;
    let world_pos = scaled_vertex + instance.center + instance.transform_position;
    
    var output: VertexOutput;
    output.position = vec4<f32>(world_pos, 0.0, 1.0);
    output.world_pos = world_pos;
    output.center = instance.center + instance.transform_position;
    output.radius = instance.radius;
    output.color = instance.color;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let distance = length(input.world_pos - input.center);
    
    if (distance > input.radius) {
        discard;
    }
    
    // Smooth edges for anti-aliasing
    let edge_smoothness = 1.0;
    let alpha = 1.0 - smoothstep(input.radius - edge_smoothness, input.radius, distance);
    
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}
