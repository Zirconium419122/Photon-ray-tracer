# Photon Ray Tracer

Photon is a simple ray tracer implemented in JavaScript and Rust. It is designed to render realistic images by simulating the behavior of light rays as they interact with objects in a 3D scene.

## Features

- Diffuse reflections (specular reflections will be added later)
- Progressive rendering for improved image quality
- Support for multiple samples per pixel
- Basic scene setup with objects and lights

### Planned Features:

1. **Refraction:** Implement support for transparent materials and simulate the refraction of light rays.

2. **Bounding Volume Hierarchy (BVH):** Introduce a BVH structure for efficient ray-object intersection tests, improving rendering performance.

3. **Texture Mapping:** Enable texture mapping to apply realistic textures to objects in the scene.

4. **Multi-Threading:** Explore multi-threading or Web Workers to parallelize ray tracing computations for faster rendering.

5. **Shaders:** Explore shaders to improve performance by the use of the GPU for faster rendering.

6. **Advanced Lighting Models:** Implement more sophisticated lighting models, such as physically-based rendering (PBR) and global illumination.

7. **Camera System:** Implement a camera system for the movement of the camera through the scene and settings like fov, focal length, and depth of field.

8. **Scene Editor:** Implement a scene editor where you can add and adjust the size and material of the objects in the scene.

9. **Ui:** A simple Ui for adjusting rendering settings, like number of samples, number of frames, and the number of reflections

### How to Contribute

If you have ideas for additional features or improvements, feel free to open an issue to discuss or create a pull request to contribute directly. Your feedback and contributions are highly appreciated!

### Feature Requests

If you have specific features you'd like to see added to Photon, please open an issue and tag it as a feature request. We'll consider it for future development.

## Prerequisites

- Web browser with support for HTML5 JavaScript and WASM
- Cargo and wasm-pack for the compilation of the Rust code
- Node.js to host the web server

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Zirconium419122/Photon-ray-tracer.git
2. Navigate to the project directory
   ```bash
   cd Photon-ray-tracer
3. Compile the Rust code into WASM and create the JavaScript glue files
   ```bash
   wasm-pack build --target web
4. Run the following command to start the development server
   ```bash
   npm run dev

## Usage

### How to add a object to the scene

Adding a object to the scene is quite simple. All you have to do is create a new object and add it to the scene. This can be done by using the `addObject` function. Let's do it with a sphere. We add it inside a scope thatway we don't have to rename the object and the parts of it each time.
```javascript
{
   const sphereCenter = new wasm.Vector(0, 0, -5);
   const sphereRadius = 1;
   const sphereMaterial = new wasm.Material(new wasm.Color(1, 0, 0));
   const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial);

   scene.addObject(sphere);
   console.log(sphere);
}
```
Firstly we create a `Vector` object to represent the center of the sphere and a number to represent the radius of the sphere in this case 1. We then create a `Material` object to represent the color of the sphere and if it emmits light or not. Finally we add the sphere to the scene and log it to the console.