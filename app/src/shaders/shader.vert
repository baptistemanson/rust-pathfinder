#version 450

layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 v_TexCoord;

// layout(set = 0, binding = 0) uniform Locals {
//     mat4 u_Transform;
// };

void main() {
    v_TexCoord = vec2(a_Pos.x/2. +0.5, 0.5-a_Pos.y/2.0);
    gl_Position = a_Pos;
}
