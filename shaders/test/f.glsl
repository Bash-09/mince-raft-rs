#version 430

in vec3 col;

out vec4 color;

void main() {
    color = vec4(col, 1.0);
}