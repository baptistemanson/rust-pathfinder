[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(1)]]
var<in> uv: vec2<f32>;
[[location(0)]]
var<out> uv_out: vec2<f32>;
[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[stage(vertex)]]
fn main() {
    uv_out = vec2<f32>(uv);
    out_position = vec4<f32>(in_position.xy, 0.2, 1.0);
}


[[location(0)]]
var<in> uv: vec2<f32>;
[[location(0)]]
var<out> out_target: vec4<f32>;

[[stage(fragment)]]
fn main() {
    var v: vec2<f32> = uv * (vec2<f32>(1.0, 1.0) - uv);
    var vig : f32 = v.x*v.y*30.0; // multiply with sth for intensity
    vig = pow(vig, 0.2); // change pow for modifying the extend of the  vignette
    out_target = vec4<f32>(0.0, 0.0, 0.0, 1.0 - vig);
    // out_target = vec4<f32>(vig, vig, vig, 1.0);
}
