#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec3 aColor;
layout (location = 3) in vec2 aTexCoord;
layout (location = 4) in float height;

out VS_OUT {
    vec3 normal;
    vec3 color;
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
    vs_out.color = height > sea_level ? (height / 1000.0) * vec3(0.0, 1.0, 0.0) : (height / 1000.0) * vec3(0.0, 0.0, 1.0);;
    vs_out.height = height;
}