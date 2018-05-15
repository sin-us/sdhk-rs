out VS_OUT {
    vec4 pos;
    vec3 color;
} vs_out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

float random (in vec2 st) {
    return fract(sin(dot(st.xy,
                         vec2(12.9898,78.233)))*
        43758.5453123);
}

float modI(float a,float b) {
    float m=a-floor((a+0.5)/b)*b;
    return floor(m+0.5);
}

float fade(float t)
{
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}

float lerp(float t, float a, float b)
{
    return a + t * (b - a);
}

float grad(int hash, float x)
{
    return (hash & 1) == 0 ? x : -x;
}

float grad(int hash, float x, float y)
{
    return ((hash & 1) == 0 ? x : -x) + ((hash & 2) == 0 ? y : -y);
}

float grad(int hash, float x, float y, float z)
{
    int h = hash & 15;
    float u = h < 8 ? x : y;
    float v = h < 4 ? y : (h == 12 || h == 14 ? x : z);
    return ((h & 1) == 0 ? u : -u) + ((h & 2) == 0 ? v : -v);
}

const int[257] perm = int[](
        151,160,137,91,90,15,
        131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,
        190, 6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,
        88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,
        77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,
        102,143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
        135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,123,
        5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,
        223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167, 43,172,9,
        129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,246,97,228,
        251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107,
        49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,
        138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180,
        151
    );

// Based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
float noise (in vec3 st) {
    float x = st.x;
    float y = st.y;
    float z = st.z;
    
    int X = int(floor(x));
    int Y = int(floor(y));
    int Z = int(floor(z));;
    x -= floor(x);
    y -= floor(y);
    z -= floor(z);
    float u = fade(x);
    float v = fade(y);
    float w = fade(z);
    int A  = (perm[X  ] + Y) & 0xff;
    int B  = (perm[X+1] + Y) & 0xff;
    int AA = (perm[A  ] + Z) & 0xff;
    int BA = (perm[B  ] + Z) & 0xff;
    int AB = (perm[A+1] + Z) & 0xff;
    int BB = (perm[B+1] + Z) & 0xff;
    return lerp(w, lerp(v, 
                        lerp(u, 
                            grad(perm[AA], x, y, z),
                            grad(perm[BA], x-1, y, z)
                        ),
                        lerp(u,
                            grad(perm[AB], x, y-1, z),
                            grad(perm[BB], x-1, y-1, z)
                        )
                    ),
                    lerp(v,
                        lerp(u, 
                            grad(perm[AA+1], x, y, z-1),
                            grad(perm[BA+1], x-1, y, z-1)),
                            lerp(u, grad(perm[AB+1], x, y-1, z-1), grad(perm[BB+1], x-1, y-1, z-1))));
}

#define OCTAVES 6
float fbm (in vec3 st) {
    // Initial values
    float value = 0.0;
    float amplitude = .5;
    float frequency = 0.;
    //
    // Loop of octaves
    for (int i = 0; i < OCTAVES; i++) {
        value += amplitude * noise(st);
        st *= 2.;
        amplitude *= .5;
    }
    return value;
}

#define TURBULENCE_OCTAVES 4
float turbulence (in vec3 st) {
    // Initial values
    float value = 0.0;
    float amplitude = .5;
    float frequency = 0.;
    //
    // Loop of octaves
    for (int i = 0; i < TURBULENCE_OCTAVES; i++) {
        value += amplitude * abs(noise(st));
        st *= 2.;
        amplitude *= .5;
    }
    return value;
}

void main()
{
    float turbulence_value = turbulence(aPos * 3.0) * 3.0;
    // turbulence_value = pow(turbulence_value, 1.5);
    gl_Position = projection * view * model * vec4(aPos + (aPos * turbulence_value / 12.0), 1.0);
    vs_out.pos = vec4(aPos, 1.0);
    vs_out.color = aColor;
}