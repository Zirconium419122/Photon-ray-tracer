# Photon Ray Tracer

Photon is a simple ray tracer implemented in JavaScript and Rust. It is designed to render realistic images by simulating the behaviour of light rays as they interact with objects in a 3D scene.

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

9. **Ui:** A simple Ui for adjusting rendering settings, like the number of samples, number of frames, and the number of reflections

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

### How to set up the boilerplate

We need three things to render the scene the first is the `scene` which can be created like so.
```javascript
const scene = new Scene();
```
Now you can add the objects to the scene as per the instructions in this section on how to add [Cubes](#how-to-add-a-cube-to-the-scene) to the scene and here how to add [Spheres](#how-to-add-a-sphere-to-the-scene) to the scene, the second thing we need is the `renderer` which can be made like so.
```javascript
const renderer = new Renderer(canvas, scene);
```
The last thing we need before we call the `render` function is to define the parameters for the renderer functions which we show how to do [here](#how-to-change-the-parameters-of-the-renderer).

### How to add a Sphere to the scene

Adding an object to the scene is quite simple. All you have to do is create a new object and add it to the scene. This can be done by using the `addObject` function. Let's do it with a sphere. We add it inside a scope that way we don't have to rename the object and the parts of it each time we want to add another one.
```javascript
{
   const sphereCenter = new wasm.Vector(0, 0, -5);
   const sphereRadius = 1;
   const sphereMaterial = new Material(new wasm.Vector(1, 0, 0));
   const sphere = new Sphere(sphereCenter, sphereRadius, sphereMaterial);

   scene.addObject(sphere);
   console.log(sphere);
}
```
Firstly we create a `Vector` object to represent the center of the sphere and a number to represent the radius of the sphere in this case `1`. We then create a `Material` object to represent the colour of the sphere and whether it emits light or not. Finally, we add the sphere to the scene and log it to the console.


### How to add a Cube to the scene

Adding a cube is basically the same as adding a sphere. But instead of a `Sphere` object we use a `Cube` object and instead of a radius we define a size which is a `Vector` where X, Y and Z determine the size of the cube/box. And we add it to the scene by using the `addObject` function after having defined the cube. This we could do like so.
```javascript
{
   const cubeCenter = new wasm.Vector(0, 0, -5);
   const cubeSize = new wasm.Vector(1, 1, 1);
   const cubeMaterial = new Material(new wasm.Vector(0, 0, 1));
   const cube = new Cube(cubeCenter, cubeSize, cubeMaterial);

   scene.addObject(cube);
   console.log(cube);
}
```
Here we first define the centre of the cube and then we define its size with a `Vector` of values X, Y and Z. We then create a `Material` object to represent the colour of the cube and if it emits light or not. Finally, we add the cube to the scene using the `addObject` function and log it to the console. The cube we've created has a size of `1` in all directions making this a cube and not a box and it's centred at `0, 0, -5` so five units in front of the Camera lastly we define that it should have a dark blue color.

### How to change the parameters of the renderer

To change the number of samples per pixel, the reflection depth or the number of frames you would change the variables defined in the `main.js` file.
```javascript
const maxReflectionDepth = 10;
const numSamples = 5;
const numFrames = 1;
```
This sets the settings to have a reflection depth of `10` and to project `5` rays and to only render `1` frame.

Though defining these as global variables might not be the optimal solution I should probably define a class/struct.
