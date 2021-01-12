#version 450
precision highp float;
precision highp int;
layout(location = 0) in vec4 a_Pos;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 0) out vec2 uv;


void main() {
    uv = vec2(a_Pos.x/2. +0.5, 0.5-a_Pos.y/2.0);
    gl_Position = a_Pos;
}
