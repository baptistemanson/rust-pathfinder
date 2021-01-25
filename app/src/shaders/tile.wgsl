[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(0)]]
var<out> world_position: vec2<f32>;
[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[block]]
struct Info {
    dim: vec2<f32>;
};

// dimension of the viewport, in number of tiles.Should be replaced by a camera to do zoom in/out and rotation?
[[group(0), binding(4)]]
var output_info: Info;

// top left corner of the viewport, in number of tiles. Should be replaced by a camera to do zoom and rotation?
[[group(0), binding(6)]]
var scroll: Info;

[[stage(vertex)]]
fn main() {
    // world_position is vertex position in tiles in the world: from [0; output_info.dim] + scroll.
    // with a projection matrix, we could feed that as an input to the whole process.
    world_position = vec2<f32>(0.5 * in_position.x + 0.5, 0.5 - 0.5 * in_position.y) * output_info.dim + scroll.dim;
    out_position = in_position;
}

[[location(0)]]
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
[[group(0), binding(5)]]
var tile_atlas_info: Info;


[[stage(fragment)]]
fn main() {

    var blueprint_position: vec2<f32> = world_position / blueprint_info.dim;
    var blueprint_value : vec4<f32> = textureSample(tile_blueprint, s, blueprint_position);
    // tile id from 0 to tile_atlas_dim.x * tile_atlas_dim.y
    var tile_id: f32 = blueprint_value.x * 255.;

    // vec2(1.,2.) means the tile at position x=1, y=2. Starting from the top.
    var tile_id_pos_in_nbtile: vec2<f32> = vec2<f32>( tile_id % tile_atlas_info.dim.x, tile_id / tile_atlas_info.dim.y);

    var tile_atlas_top_left: vec2<f32> = floor(tile_id_pos_in_nbtile) / tile_atlas_info.dim;

    // here it is assumed that 1 tile = 1 unit.
    var position_in_tile_atlas:vec2<f32> = tile_atlas_top_left +  fract(world_position) / tile_atlas_info.dim;
    var out_val: vec4<f32> = textureSample(tile_atlas, s, position_in_tile_atlas);
    out_target = vec4<f32>( out_val, 1.0);
}
