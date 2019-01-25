#version 330
in vec2 position;
out vec2 vPos;

uniform vec2 offset;
uniform float scale;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vPos = position.xy * scale + offset;
}
