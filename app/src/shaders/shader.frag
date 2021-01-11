#version 450
precision highp float;
precision highp int;
layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform texture2D t_tile;
layout(set = 0, binding = 2) uniform sampler s;
layout(set = 0, binding = 3) uniform texture2D t_tile_index;

vec2 nb_tiles = vec2(8., 6.);
vec2 tile_dim = 1. / nb_tiles;

void main() {
    vec4 tile_index = texture(sampler2D(t_tile_index, s), v_TexCoord);
    float to_pick = tile_index.r*255.;
    vec2 tile_to_pick = vec2( mod(to_pick,nb_tiles.x), floor(to_pick/nb_tiles.y));
    vec2 inner_tile = mod(v_TexCoord.xy, tile_dim) + tile_to_pick * tile_dim;
     vec4 tex = texture(sampler2D(t_tile, s), inner_tile);
    o_Target = vec4(tex.rgb,1.0);
}
