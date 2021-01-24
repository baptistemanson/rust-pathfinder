[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(0)]]
var<out> uv: vec2<f32>;
[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[stage(vertex)]]
fn main() {
    uv = vec2<f32>(0.5 * in_position.x + 0.5, 0.5 - 0.5 * in_position.y);
    out_position = in_position;
}

[[location(0)]]
var<in> uv: vec2<f32>;
[[location(0)]]
var<out> out_target: vec4<f32>;

[[group(0), binding(1)]]
var tile_atlas: texture_2d<f32>;

[[group(0), binding(0)]]
var s: sampler;


[[stage(fragment)]]
fn main() {
    var out_val: vec4<f32> = vec4<f32>(1.0,0.,0.,0.5);
    out_target = out_val;
}
