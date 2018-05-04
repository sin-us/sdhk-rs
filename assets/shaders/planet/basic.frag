#version 330 core
in GS_OUT {
    vec3 normal;
    vec3 color;
} fs_in;

out vec4 FragColor;

uniform sampler2D texture0;
uniform sampler2D texture1;
uniform float mixAmount;

uniform vec3 light_direction;

void main()
{
    float light = max(dot(fs_in.normal, light_direction), 0.1);
    FragColor = vec4(light * fs_in.color, 1.0); // mix(texture(texture0, texCoord), texture(texture1, texCoord), mixAmount);
}