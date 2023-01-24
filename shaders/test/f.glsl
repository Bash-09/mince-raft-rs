#version 430

in vec3 tex;
uniform sampler2DArray textures;
out vec4 color;


void main() {
    vec4 texCol = texture(textures, tex);
    
    if (texCol.a < 0.5) {
        discard;
    }

    color = texCol;
}
