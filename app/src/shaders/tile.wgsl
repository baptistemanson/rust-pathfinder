[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(1)]]
var<in> uv: vec2<f32>;

[[location(0)]]
var<out> blueprint_position: vec2<f32>;
[[location(1)]]
var<out> world_position: vec2<f32>;

[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[block]]
struct Info {
    dim: vec2<f32>;
};


// top left corner of the viewport, in number of tiles. Should be replaced by a camera to do zoom and rotation?
[[block]]
struct Locals {
    transform: mat4x4<f32>;
};
[[group(0), binding(5)]]
var r_locals: Locals;

[[stage(vertex)]]
fn main() {
    blueprint_position = vec2<f32>(uv);
    world_position = vec2<f32>(in_position.xy);
    out_position = r_locals.transform * in_position;
}


[[location(0)]]
var<in> blueprint_position: vec2<f32>;
[[location(1)]]
var<in> world_position: vec2<f32>;

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

// size of the blueprint, in number of tiles
[[group(0), binding(3)]]
var blueprint_info: Info;

// size of the atlas, in number of tiles
[[group(0), binding(4)]]
var tile_atlas_info: Info;


[[stage(fragment)]]
fn main() {
    var blueprint_value : vec4<f32> = textureSample(tile_blueprint, s, blueprint_position);
    // tile id from 0 to tile_atlas_dim.x * tile_atlas_dim.y
    var tile_id: f32 = blueprint_value.x;

    // vec2(1.,2.) means the tile at position x=1, y=2. Starting from the top.
    var tile_id_pos_in_nbtile: vec2<f32> = vec2<f32>( tile_id % tile_atlas_info.dim.x, tile_id / tile_atlas_info.dim.y);

    var tile_atlas_top_left: vec2<f32> = floor(tile_id_pos_in_nbtile) / tile_atlas_info.dim;

    // here it is assumed that 1 tile = 1 unit.
    var position_in_tile_atlas:vec2<f32> = tile_atlas_top_left + fract(vec2<f32>(world_position.x, 0.0 - world_position.y)) / tile_atlas_info.dim;
    var out_val: vec4<f32> = textureSample(tile_atlas, s, position_in_tile_atlas);
     out_target = vec4<f32>(out_val,1.0);
}
