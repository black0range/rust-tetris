#version 330

in vec3 position;

uniform mat4 camera_mat;
uniform mat4 model_mat;


void main() {
  gl_Position = camera_mat * model_mat * vec4(position.xyz, 1.0);
}
