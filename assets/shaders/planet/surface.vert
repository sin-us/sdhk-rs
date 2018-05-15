out VS_OUT {
    vec3 normal;
    vec2 texCoord;
    float height;
} vs_out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float sea_level;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    vs_out.normal = aNormal;
    vs_out.texCoord = aTexCoord;
    vs_out.height = aHeight;
}