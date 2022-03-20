#version 430

in vec3 position;

uniform mat4 pvmat;
uniform mat4 tmat;

void main() {
    vec4 world_pos = tmat * vec4(position, 1.0);
    vec4 pos = pvmat * world_pos;
    gl_Position = pos;
}