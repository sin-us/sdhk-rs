in VS_OUT {
    vec3 normal;
    vec3 color;
    vec2 texCoord;
    float height;
    float clouds;
} fs_in;

out vec4 FragColor;

void main()
{
    FragColor = fs_in.clouds > 0.0 ? vec4(1.0, 1.0, 1.0, 0.6) : vec4(0.0, 0.0, 0.0, 0.0);
}