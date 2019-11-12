#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 0) out vec2 v_uv;

layout(set = 0, binding = 0) uniform UboInstance {
    mat4 mvp;
} uboInstance;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    v_uv = a_uv;
    vec4 mvp_pos =  uboInstance.mvp * vec4(a_pos,1);
    gl_Position = mvp_pos;
}