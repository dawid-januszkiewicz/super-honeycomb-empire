// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}


    // layout (location = 0) in vec2 in_cube;
    // layout (location = 1) in vec3 in_color;
    // uniform float orientation[9];
    // uniform vec2 size;
    // uniform vec2 origin;
    // float M[9] = orientation;
    // float x = (M[0] * in_cube[0] + M[1] * in_cube[1]) * size.x;
    // float y = (M[2] * in_cube[0] + M[3] * in_cube[1]) * size.y;
    // vec2 pos = vec2(x, y);
    
    // out vec3 v_color;
    // void main() {
    //     gl_Position = vec4(pos + origin, 0.0, 1.0);
    //     v_color = in_color;
    // }
