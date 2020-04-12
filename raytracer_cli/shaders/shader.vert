#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texcoords;

layout(location = 0) out vec2 out_tc;

out gl_PerVertex {
  vec4 gl_Position;
};

void main() {
    gl_Position = position;
    out_tc = texcoords;
}
