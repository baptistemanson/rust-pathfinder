// simple.vert
#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 vertex_position;
layout(location = 0) out vec4 varying_color;

void main() {
    varying_color = vec4(vertex_position, 1.0);
    gl_Position = vec4(vertex_position, 1.0);
}
