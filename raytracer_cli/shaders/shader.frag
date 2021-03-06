#version 450

layout(location = 0) in vec2 texcoords;

layout(location = 0) out vec4 color;

layout(set = 0, binding = 0) uniform texture2D t_Color;
layout(set = 0, binding = 1) uniform sampler s_Color;

void main() {
    color = texture(sampler2D(t_Color, s_Color), texcoords);
}
