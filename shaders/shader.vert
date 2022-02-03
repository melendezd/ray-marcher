#version 450

uniform mat4 view;

in vec2 position;
in vec3 color;

out vec3 vColor;
out vec2 vPos;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);// * matrix;
    vPos = position;
    vColor = color;
}
