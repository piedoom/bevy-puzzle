#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};
struct Time {
    time: f32;
};
[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> base_time: Time;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.uv = vertex.uv;
    out.position = view.view_proj * world_position;
    return out;
}

fn fs_rot(angle: f32) -> mat2x2<f32> {
    return mat2x2<f32>(
        vec2<f32>(cos(angle), -sin(angle)),
        vec2<f32>(sin(angle), cos(angle)),
    );
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let speed = 0.5;
    let scale = 500.;
    let multiplier = 0.05;

    let time = base_time.time * speed;
    // let dir = vec3<f32>(in.uv,1.0);
    // return vec4<f32>(dir.x/in.clip_position.x,in.clip_position.y,1.0,1.0);
    let r_angle = sin(time / 1.4) / 3.;
    let ra = fs_rot(r_angle) * (in.position.xy / scale);
    let r = sin(ra.x * 3.) / 2. + 0.5;

    let g_angle = sin(time / 1.9) / 2.;
    let ga = fs_rot(g_angle) * (in.position.xy / scale);
    let g = sin(ga.x * 4.) / 2. + 0.5;

    let b_angle = sin(time / 1.6) / 1.2;
    let ba = fs_rot(b_angle) * (in.position.xy / scale);
    let b = sin(ba.x * 5.) / 2. + 0.5;

    return vec4<f32>(r * multiplier, g * multiplier * 0.2, b * multiplier, 0.2);
}
