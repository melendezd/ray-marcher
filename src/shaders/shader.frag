#version 450

uniform float near;
uniform mat3 cam_rot;
uniform vec3 cam_pos;
uniform float aspect_ratio;

uniform float time;

uniform usampler3D sdf_data;
uniform usampler3D voxels;

#define MAX_STEPS 1000
#define STEP_SIZE 0.01
#define EPS       1e-4

in vec3 vColor;
in vec2 vPos;

out vec4 f_color;

#define PI 3.14159265358979

#define VOX_RED   1
#define VOX_BLUE  2
#define VOX_GREEN 3

const ivec3 sdf_size = textureSize(sdf_data, 0);

int sdf(vec3 p);
int voxel(vec3 p);

vec3 next_point(vec3 pos, vec3 dir);

vec3 hsv2rgb(vec3 c);
vec3 rgb2hsv(vec3 c);

void main() {
    //float res = 200;
    //vec2 pos = floor(vPos * (res / 2)) / (res / 2);
    vec2 pos = vPos;
    vec3 dir_rel = vec3(pos.x, pos.y / aspect_ratio, near);
    vec3 dir = normalize(cam_rot * dir_rel);

    vec3 v = cam_pos;

    for (int step = 0; step < MAX_STEPS; step++) {
        // l1 distance from current voxel to the nearest filled voxel
        int l1_dist = int(sdf(v));
        if (l1_dist > 0) {
            for (int i = 0; i < l1_dist; i++) {
                v = next_point(v, dir);
            }
        } else {
            // hit a voxel! give it a pretty color for now
            vec3 rel = v - cam_pos;
            float dist = dot(rel, rel);
            float light_factor = ((STEP_SIZE * MAX_STEPS) / (dist/10));
            //vec4 fun_color = abs(0.2 * vec4(normalize(v - cam_pos), 1));
            vec4 base_color = vec4(0,0,0,0);
            unsigned int vox = voxel(v);

            if (vox == VOX_RED)
                base_color = vec4(1, 0, 0, 1);
            else if (vox == VOX_BLUE)
                base_color = vec4(0, 0, 1, 1);
            else if (vox == VOX_GREEN)
                base_color = vec4(0, 1, 0, 1);
            else
                base_color = vec4(1, 0, 1, 1);

            f_color = abs(light_factor *  base_color);

            vec3 hsv = rgb2hsv(f_color.rgb);
            hsv.y = 0.99;
            f_color.rgb = hsv2rgb(hsv);
            return;
        }
    }

    // found nothing after MAX_STEPS steps
    f_color = vec4(0.01,0,0,1);
}


// Gives the next point after stepping through one voxel (to the boundary), starting at *pos*, in direction *dir*
vec3 next_point(vec3 pos, vec3 dir) {
    // next_pts[i] is the ith component of the vector obtained after stepping from pos in direction dir until we hit a voxel boundary where component i is 0
    // this assumes that the voxels are 1x1x1 units
    vec3 next_pts = sign(dir) * ceil(sign(dir) * pos);
    //vec3 next_pts = ceil(pos);

    // step_size_v.x, step_size_v.y, step_size_v.z are the smallest step sizes to make the x,y,z components zero, respectively
    vec3 step_size_v = (next_pts - pos) / dir;

    // the desired step size (step size to step to nearest voxel boundary) is the smallest component of step_size_v
    float step_size = min(step_size_v.x, min(step_size_v.y, step_size_v.z));

    return pos + (step_size + 0.01) * dir;
}

// Underestimator of the l1 distance from voxel containing p to the nearest filled voxel
int sdf(vec3 p) {
    if (any(greaterThanEqual(p, sdf_size - 1)) || any(lessThan(p, vec3(0)))) {
        return 0;
    }
    return int(texelFetch(sdf_data, ivec3(p), 0).r);
}

// Gets the id (voxel type) of the voxel at the given position
int voxel(vec3 p) {
    if (any(greaterThanEqual(p, sdf_size - 1)) || any(lessThan(p, vec3(0)))) {
        return 0;
    }
    return int(texelFetch(voxels, ivec3(p), 0).r);
}


////////////////////////////////////////////////////////////////////////////////
// color util functions                                                       //
////////////////////////////////////////////////////////////////////////////////

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
