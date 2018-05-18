out VS_OUT {
    vec3 normal;
    vec2 texCoord;
    float brightness;
    float temperature;
    float height;
} vs_out;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float sea_level;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    vs_out.normal = vec3(model * vec4(aNormal, 1.0));
    vs_out.texCoord = aTexCoord;

    vs_out.brightness = aBrightness;
    vs_out.temperature = aTemperature;
    vs_out.height = aHeight;
}