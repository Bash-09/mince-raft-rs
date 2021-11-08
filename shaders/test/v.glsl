#version 430

in vec3 position;
out vec3 col;

uniform mat4 pvmat;
uniform mat4 tmat;

void main() {
    col = position / 16.0;
    vec4 world_pos = tmat * vec4(position, 1.0);
    vec4 pos = pvmat * world_pos;
    gl_Position = pos;
}