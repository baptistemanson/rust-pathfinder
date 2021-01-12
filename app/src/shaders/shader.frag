#version 450
precision highp float;
precision highp int;
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform texture2D tile_atlas;
layout(set = 0, binding = 2) uniform sampler s;
layout(set = 0, binding = 3) uniform texture2D tile_blueprint;

// dimension of the tile atlas, in number of tiles
layout(set = 0, binding = 0) uniform MainBlock {
    vec2 tile_atlas_dim;
};
vec2 tile_atlas_len = 1. / tile_atlas_dim;

// dimension of the output grid
vec2 tile_output_dim = vec2(4.,4.);
vec2 tile_output_len = 1. / tile_output_dim;

vec2 ratio = tile_atlas_len/tile_output_len;
void main() {
    // tile number from 0 to tile_atlas_dim.x * tile_atlas_dim.y
    float tile_nb = texture(sampler2D(tile_blueprint, s), uv).r *255.;
    // vec2(1.,2.) means the tile at position x=1, y=2. Starting from the top.
    vec2 tile_coordinates_in_tile = vec2( mod(tile_nb,tile_atlas_dim.x), floor(tile_nb/tile_atlas_dim.y));

    vec2 tile_atlas_top_left = tile_coordinates_in_tile * tile_atlas_len;
    vec2 position_in_tile_atlas = tile_atlas_top_left + mod(uv.xy, tile_output_len) * ratio;
    //o_Target = vec4(mod(uv.xy, tile_output_len), 0.,1.0);

    vec4 tex = texture(sampler2D(tile_atlas, s), position_in_tile_atlas);
    o_Target = vec4(tex.rgb,1.0);
}
