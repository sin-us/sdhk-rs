out VS_OUT {
    vec3 normal;
    vec3 color;
    vec2 texCoord;
    float height;
    float clouds;
} vs_out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float sea_level;

void main()
{
    gl_Position = projection * view * model * vec4(aPos * 1.1, 1.0);
    vs_out.normal = aNormal;
    vs_out.texCoord = aTexCoord;
    vs_out.color = aHeight > sea_level ? (aHeight / 1000.0) * vec3(0.0, 1.0, 0.0) : (aHeight / 1000.0) * vec3(0.0, 0.0, 1.0);;
    vs_out.height = aHeight;
    vs_out.clouds = aClouds;
}