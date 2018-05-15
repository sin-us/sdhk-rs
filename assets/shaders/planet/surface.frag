// in GS_OUT {
//     vec3 normal;
//     vec3 color;
// } fs_in;

in VS_OUT {
    vec3 normal;
    vec2 texCoord;
    float height;
} fs_in;

out vec4 FragColor;

uniform float sea_level;
uniform vec3 light_direction;

void main()
{
    vec3 color = fs_in.height > sea_level ? (fs_in.height / 1000.0) * vec3(0.0, 1.0, 0.0) : (fs_in.height / 1000.0) * vec3(0.0, 0.0, 1.0);
    float light = max(dot(fs_in.normal, light_direction), 0.1);
    FragColor = vec4(light * color, 1.0);
}