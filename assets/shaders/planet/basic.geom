#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices = 24) out;

in VS_OUT {
    vec3 normal;
    vec3 color;
    vec2 texCoord;
    float height;
} gs_in[];

out GS_OUT {
    vec3 normal;
    vec3 color;
} gs_out;

uniform float sea_level;

vec3 GetNormal()
{
   vec3 a = vec3(gl_in[0].gl_Position) - vec3(gl_in[1].gl_Position);
   vec3 b = vec3(gl_in[2].gl_Position) - vec3(gl_in[1].gl_Position);
   return normalize(cross(a, b));
}

void Emit(int i, vec4 height) {
    gl_Position = gl_in[i].gl_Position + height;
    gs_out.normal = gs_in[i].normal;
    gs_out.color = gs_in[i].color;
    EmitVertex();
}

void main() {
    vec3 normal = GetNormal();
    vec4 height = vec4(normal * (gs_in[0].height > sea_level ? gs_in[0].height / 1000.0 : sea_level / 1000.0), 0.0);
    vec4 zero = vec4(0.0);

    Emit(0, zero); // A
    Emit(1, zero); // B
    Emit(2, zero); // C

    Emit(2, height);
    Emit(0, zero);
    Emit(0, height);
    Emit(1, zero);

    Emit(1, height);
    Emit(2, height);
    Emit(0, height);
    
    EndPrimitive();
}  