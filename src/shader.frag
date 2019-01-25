#version 330
in vec2 vPos;
out vec4 f_color;

const int max_iteration = 255;

void main() {
    float real = 0.0;
    float imag = 0.0;

    int i = 0;
    for(; real*real + imag*imag <= 4 && i < max_iteration; i++) {
        float real_temp = real*real - imag*imag + vPos.x;
        imag = 2*real*imag + vPos.y;
        real = real_temp;
    }

    float c = float(i) / max_iteration;
    f_color = vec4(vec3(c), 1.0);
}
