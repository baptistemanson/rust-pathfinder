#version 450
precision highp float;
precision highp int;
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o_Target;

void main() {
    float factor = length(uv)/2.;
    o_Target = vec4(1.,0.,0.,factor);
}
