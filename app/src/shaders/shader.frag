#version 450
precision highp float;
precision highp int;
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform texture2D tile_atlas;
layout(set = 0, binding = 0) uniform sampler s;
layout(set = 0, binding = 2) uniform texture2D tile_blueprint;



// dimension of the output grid
layout(set = 0, binding = 3) uniform MainBlock2 {
    vec2 blueprint_dim;
};
vec2 blueprint_len = 1. / blueprint_dim;

layout(set = 0, binding = 4) uniform MainBlock3 {
    vec2 output_dim;
};

// dimension of the tile atlas, in number of tiles
layout(set = 0, binding = 5) uniform MainBlock {
    vec2 tile_atlas_dim;
};
vec2 tile_atlas_len = 1. / tile_atlas_dim;

// scroll is in number of tiles
layout(set = 0, binding = 6) uniform MainBlock4 {
    vec2 scroll;
};

vec2 output_len = 1. / output_dim;

vec2 ratio_output = output_dim / tile_atlas_dim;

vec2 ratio_lookup = output_dim / blueprint_dim;

void main() {
    vec2 position_in_blueprint =  uv*ratio_lookup + scroll*blueprint_len;
    // tile number from 0 to tile_atlas_dim.x * tile_atlas_dim.y
    float tile_nb = texture(sampler2D(tile_blueprint, s),position_in_blueprint).r *255.;
    // vec2(1.,2.) means the tile at position x=1, y=2. Starting from the top.
    vec2 tile_coordinates_in_tile = vec2( mod(tile_nb,tile_atlas_dim.x), tile_nb/tile_atlas_dim.y);
    vec2 tile_atlas_top_left = floor(tile_coordinates_in_tile) * tile_atlas_len;
    vec2 position_in_tile_atlas = tile_atlas_top_left +  tile_atlas_len * fract(scroll+uv.xy* output_dim);
    vec4 tex = texture(sampler2D(tile_atlas, s), position_in_tile_atlas);
    o_Target = vec4(tex.rgb,1.0);
}
