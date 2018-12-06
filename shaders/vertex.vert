#version 330

in vec3 position;

uniform mat4 camera_projection;

void main() {
  gl_Position = camera_projection * vec4(position.xy, 0.0, 1.0);
}
