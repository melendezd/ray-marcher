#version 450

uniform float near;
uniform mat3 cam_rot;
uniform vec3 cam_pos;
uniform float aspect_ratio;

uniform float time;

uniform usampler3D sdf_data;

// width : height
//uniform float aspect_ratio;

#define MAX_STEPS 1000
#define STEP_SIZE 0.01
#define EPS       1e-4

in vec3 vColor;
in vec2 vPos;

out vec4 f_color;

#define PI 3.14159265358979

float sdf(vec3 p);
vec3 hsv2rgb(vec3 c);
vec3 rgb2hsv(vec3 c);

void main() {
    float res = 200;
    vec2 pos = floor(vPos * (res / 2)) / (res / 2);
    vec3 dir_rel = vec3(pos.x, pos.y / aspect_ratio, near);
    vec3 dir = normalize(cam_rot * dir_rel);

    vec3 v = cam_pos;

    float step_size = 0;
    for(int i = 0; i < MAX_STEPS; i++) {
        step_size = sdf(v);
        /*
        if (step_size == 1) {
            f_color = vec4(0, 1, 0, 0);
            return;
        }
        */
        if (step_size <= EPS) {
            vec3 rel = v - cam_pos;
            float dist = dot(rel, rel);
            f_color = vec4(normalize(v - cam_pos), 1);
            f_color = ((STEP_SIZE * MAX_STEPS) / dist) * f_color;
            f_color = abs(f_color);

            vec3 hsv = rgb2hsv(f_color.rgb);
            hsv.y = 0.99;
            f_color.rgb = hsv2rgb(hsv);

            //f_color = vec4(1, 0, 0, 0);
            return;
        }
        v += dir * step_size;
    }
    f_color = vec4(0.01,0,0,1);
}

float sdf(vec3 p) {
    return float(texelFetch(sdf_data, ivec3(p * 8), 0)) * STEP_SIZE;
}

/*
float sdf(vec3 p) {
    float pd = 5;
    float x = mod(p.x, pd);
    float y = mod(p.y, pd);
    float z = mod(p.z, pd);
    if ( -1.0 < y && y <  1.0
       && 2.0 < x && x <  3.0 
       && 2.0 < z && z <  3.0
    )  {
        return 0;
    }
    return STEP_SIZE;
}
*/

// All components are in the range [0…1], including hue.
vec3 rgb2hsv(vec3 c)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

// All components are in the range [0…1], including hue.
vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
