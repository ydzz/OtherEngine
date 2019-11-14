#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 1, binding = 0) uniform texture2D u_texture;
layout(set = 1, binding = 1) uniform sampler u_sampler;



void main() {
  target0 = texture(sampler2D(u_texture, u_sampler), v_uv);
  target0 = vec4(1.0f,0.0,0.0,1.0);
}