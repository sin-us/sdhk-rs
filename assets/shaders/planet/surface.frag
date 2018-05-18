// in GS_OUT {
//     vec3 normal;
//     vec3 color;
// } fs_in;

in VS_OUT {
    vec3 normal;
    vec2 texCoord;
    float brightness;
    float temperature;
    float height;
} fs_in;

out vec4 FragColor;

uniform float sea_level;
uniform vec3 light_direction;
uniform int overlay;

void main()
{
    if (overlay == 3) { // Temperature
        vec3 temperatureColor = fs_in.temperature > 0.0 ? vec3(fs_in.temperature / 100.0 + 0.1, 0.0, 0.0) : vec3(0.0, 0.0, -fs_in.temperature / 100.0 + 0.1);
        FragColor = vec4(temperatureColor, 1.0);
    }
    else if (overlay == 2) { // Brightness
        FragColor = vec4(vec3(fs_in.brightness), 1.0);
    } else { // Basic
        vec3 color = fs_in.height > sea_level ? (fs_in.height / 1000.0) * vec3(0.0, 1.0, 0.0) : (fs_in.height / 1000.0) * vec3(0.0, 0.0, 1.0);
        float light = max(dot(fs_in.normal, light_direction), 0.1);
        FragColor = vec4(light * color, 1.0);
    }
}