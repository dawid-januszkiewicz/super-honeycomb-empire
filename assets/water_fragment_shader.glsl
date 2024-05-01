#version 100
precision lowp float;

varying vec2 uv;
varying vec2 uv_screen;
varying float time;

uniform lowp float TileSize;
uniform lowp float Time;

void main() {
    vec2 offset = vec2(
        sin(uv.y * 10.0 + Time * 10.0),
        cos(uv.x * 10.0 + Time * 15.0)
    );

    vec2 uv_scroll = uv + offset * 0.02;

    // Make the water effect tile-able
    vec2 tiled_uv = mod(uv_scroll, 1.0);

    // Apply water effect based on tiled UV coordinates and time
    vec4 color = vec4(0.0, 0.6, 1.0, 0.8); // Water color

    // Add a different ripple effect
    float ripple = sin(tiled_uv.x * 20.0 + Time * 5.0);
    color.rgb *= 1.0 + ripple * 0.1;

    gl_FragColor = color;
}
