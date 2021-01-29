[[location(0)]]
var<in> in_position: vec2<f32>;
[[location(1)]]
var<in> uv_in: vec2<f32>;
[[location(2)]]
var<in> z: f32;

[[location(0)]]
var<out> uv_var: vec2<f32>;

[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[stage(vertex)]]
fn main() {
    uv_var = uv_in;
    out_position = vec4<f32>(in_position.x, in_position.y, z, 1.0);
}

[[location(0)]]
var<in> uv_var: vec2<f32>;

[[location(0)]]
var<out> out_target: vec4<f32>;
[[group(0), binding(1)]]
var sprite_atlas: texture_2d<f32>;

[[group(0), binding(0)]]
var s: sampler;

[[stage(fragment)]]
fn main() {
    var out_val: vec4<f32> = textureSample(sprite_atlas, s, uv_var);
    // var out_val: vec4<f32> = vec4<f32>(1.0,0.,0.,1.);
    out_target = out_val;
}
