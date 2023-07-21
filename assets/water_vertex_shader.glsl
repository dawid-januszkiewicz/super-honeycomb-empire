#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;
varying lowp vec2 uv_screen;
varying float time;

uniform mat4 Model;
uniform mat4 Projection;

uniform lowp float Time;

void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;
    time = Time;
    gl_Position = res;
}
