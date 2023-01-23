#version 430

in vec3 tex;
uniform sampler2DArray textures;
uniform float gamma;
out vec4 color;


void main() {
    vec4 texCol = texture(textures, tex);
    
    if (texCol.a < 0.5) {
        discard;
    }

    vec3 col = texCol.rgb;
    col = col / (col + vec3(1));
    col = pow(col, vec3(1.0 / gamma));

    color = vec4(col, 1.0);
}
