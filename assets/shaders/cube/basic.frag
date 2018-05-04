#version 330 core
in GS_OUT {
    vec3 color;
} fs_in;

out vec4 FragColor;

uniform sampler2D texture0;
uniform sampler2D texture1;

void main()
{
    FragColor = vec4(fs_in.color, 1.0); // mix(texture(texture0, texCoord), texture(texture1, texCoord), mixAmount);
}