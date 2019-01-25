#version 330
in vec2 vPos;
out vec4 f_color;

const int max_iteration = 100;

void main() {
    float real = 0.0;
    float imag = 0.0;

    int i = 0;
    for(; real*real + imag*imag <= 4 && i < max_iteration; i++) {
        float real_temp = real*real - imag*imag + vPos.x;
        imag = 2*real*imag + vPos.y;
        real = real_temp;
    }

    vec3 red = vec3(1, 0, 0);
    vec3 green = vec3(0, 1, 0);
    vec3 blue = vec3(0, 0, 1);
    vec3 white = vec3(1, 1, 1);

    if (i < 25) {
        float factor = float(i) / 25.0;
        f_color = vec4(factor * blue, 1.0);
    } else if (i < 50) {
        float factor = float(i - 25) / 25.0;
        f_color = vec4(factor * green + (1.0 - factor) * blue, 1.0);
    } else if (i < 75) {
        float factor = float(i - 50) / 25.0;
        f_color = vec4(factor * red + (1.0 - factor) * green, 1.0);
    } else {
        float factor = float(i - 75) / 25.0;
        f_color = vec4(factor * white + (1.0 - factor) * red, 1.0);
    }
}
