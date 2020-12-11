precision mediump float;

uniform float u_time;

void main() {
    float r = sin(u_time * 0.0003);
    float g = sin(u_time * 0.0005);
    float b = sin(u_time * 0.0007);

    gl_FragColor = vec4(r, g, b, 1.0);
}
