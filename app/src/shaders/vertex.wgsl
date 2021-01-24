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
