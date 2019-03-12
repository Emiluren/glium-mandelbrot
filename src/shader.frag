#version 330
in vec2 vPos;
out vec4 f_color;

const int max_iteration = 1024;

void main() {
    vec2 z = vec2(0, 0);

    int i = 0;
    for(; z.x*z.x + z.y*z.y <= 4 && i < max_iteration; i++) {
        z = vec2(z.x*z.x - z.y*z.y, 2*z.x*z.y) + vPos;
    }

    vec3 red = vec3(1, 0, 0);
    vec3 green = vec3(0, 1, 0);
    vec3 blue = vec3(0, 0, 1);
    vec3 yellow = vec3(1, 1, 0);

    if (i == max_iteration) {
        f_color = vec4(vec3(0), 1);
    } else {
        float log_zn = log2(dot(z, z)) / 2;
        float nu = log2(log_zn);
        float log_iters = mod((i + 1 - nu) / 10, 10);
        if (log_iters < 2.5) {
            float factor = log_iters / 2.5;
            f_color = vec4(factor * blue + (1.0 - factor) * yellow, 1.0);
        } else if (log_iters < 5.0) {
            float factor = (log_iters - 2.5) / 2.5;
            f_color = vec4(factor * green + (1.0 - factor) * blue, 1.0);
        } else if (log_iters < 7.5) {
            float factor = (log_iters - 5.0) / 2.5;
            f_color = vec4(factor * red + (1.0 - factor) * green, 1.0);
        } else {
            float factor = (log_iters - 7.5) / 2.5;
            f_color = vec4(factor * yellow + (1.0 - factor) * red, 1.0);
        }
    }
}
