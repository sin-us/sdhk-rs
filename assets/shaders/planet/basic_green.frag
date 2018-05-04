#version 330 core
in vec2 texCoord;

out vec4 FragColor;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float mixAmount;

void main()
{
    FragColor = vec4(0.0, 1.0, 0.0, 0.2);
}