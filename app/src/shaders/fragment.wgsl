[[location(0)]]
var<in> uv: vec2<f32>;
[[location(0)]]
var<out> out_target: vec4<f32>;

[[group(0), binding(1)]]
var tile_atlas: texture_2d<f32>;

[[group(0), binding(0)]]
var s: sampler;

[[group(0), binding(2)]]
var tile_blueprint: texture_2d<f32>;

[[block]]
struct Info {
    dim: vec2<f32>;
};
[[group(0), binding(3)]]
var blueprint_info: Info;
[[group(0), binding(4)]]
var output_info: Info;
[[group(0), binding(5)]]
var tile_atlas_info: Info;
[[group(0), binding(6)]]
var scroll: Info;

[[stage(fragment)]]
fn main() {
    // const uniform expressions
    var ratio_lookup: vec2<f32> = output_info.dim / blueprint_info.dim;

    var position_in_blueprint: vec2<f32> = uv.xy * ratio_lookup.xy + scroll.dim.xy / blueprint_info.dim.xy;
    var sampled : vec4<f32> = textureSample(tile_blueprint, s, position_in_blueprint);
    var tile_id: f32 = sampled.x * 255.;
    var tile_id_pos_in_nbtile: vec2<f32> = vec2<f32>( tile_id % tile_atlas_info.dim.x, tile_id / tile_atlas_info.dim.y);
    var tile_atlas_top_left: vec2<f32> = floor(tile_id_pos_in_nbtile) / tile_atlas_info.dim;
    var position_in_tile_atlas:vec2<f32> = tile_atlas_top_left +  fract(scroll.dim + uv.xy * output_info.dim) / tile_atlas_info.dim;
  
    // debug
    var out_val: vec4<f32> = textureSample(tile_atlas, s, position_in_tile_atlas);
    out_target = vec4<f32>( out_val.x , out_val.y , out_val.z, 1.0);
}
