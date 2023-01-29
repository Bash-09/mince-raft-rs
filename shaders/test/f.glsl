#version 430

in vec3 tex;
in vec3 pos;

uniform sampler2DArray textures;
uniform vec4 fogCol;
uniform float fogNear;
uniform float fogFar;

out vec4 color;

void main() {
    vec4 texCol = texture(textures, tex);
    
    if (texCol.a < 0.5) {
        discard;
    }

    float fogDistance = length(pos);
    float fogAmount = smoothstep(fogNear, fogFar, fogDistance);

    color = mix(texCol, fogCol, fogAmount);
}
