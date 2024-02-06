import './style.css'
import init, * as wasm from "./pkg/raytracer.js"

await init();

// Get the canvas element and the 2d context
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

// Set the width and height of the canvas
canvas.width = 600;  // Replace 800 with your desired width
canvas.height = 600; // Replace 600 with your desired height

// Create ImageData object for direct pixel manipulation
const imageData = ctx.createImageData(canvas.width, canvas.height);
const data = imageData.data;

async function run() {
    // Define the settigns of the renderer
    const maxReflectionDepth = 10;
    const numSamples = 5;
    const numFrames = 1;
    const settings = new wasm.Settings(maxReflectionDepth, numSamples, numFrames);

    // Add the light source 
    const sphereCenter = new wasm.Vector(-5, -5, -10);
    const sphereRadius = 5;
    const sphereMaterial = new wasm.Material(
      new wasm.Vector(0, 0, 0),
      1,
      new wasm.Vector(1, 1, 1),
      2
    );
    const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial)

    console.log(sphere);

    // Example usage
    const sphereCenter1 = new wasm.Vector(0, 0, -5);
    const sphereRadius1 = 1;
    const sphereMaterial1 = new wasm.Material(new wasm.Vector(1, 0, 0), 1, new wasm.Vector(0, 0, 0), 0);
    const sphere1 = new wasm.Sphere(sphereCenter1, sphereRadius1, sphereMaterial1);

    console.log(sphere1);

    const sphereCenter2 = new wasm.Vector(3, 1, -11);
    const sphereRadius2 = 1;
    const sphereMaterial2 = new wasm.Material(new wasm.Vector(0, 1, 0), 1, new wasm.Vector(0, 0, 0), 0);
    const sphere2 = new wasm.Sphere(sphereCenter2, sphereRadius2, sphereMaterial2);

    console.log(sphere2);

    const sphereCenter3 = new wasm.Vector(0, 5, -5);
    const sphereRadius3 = 4.5;
    const sphereMaterial3 = new wasm.Material(new wasm.Vector(0.5, 0.5, 0.5), 1, new wasm.Vector(0, 0, 0), 0);
    const sphere3 = new wasm.Sphere(sphereCenter3, sphereRadius3, sphereMaterial3);

    console.log(sphere3);

    const cubeCenter = new wasm.Vector(-2, 1, -5);
    const cubeSize = new wasm.Vector(1, 1, 1);
    const cubeMaterial = new wasm.Material(new wasm.Vector(0, 0, 1), 1, new wasm.Vector(0, 0, 0), 0);
    const cube = new wasm.Cube(cubeCenter, cubeSize, cubeMaterial);

    console.log(cube);

    const scene = new wasm.Scene();
    scene.add_sphere(sphere);
    scene.add_sphere(sphere1);
    scene.add_sphere(sphere2);
    scene.add_sphere(sphere3);
    scene.add_cube(cube);

    // Create the renderer
    const renderer = new wasm.Renderer(canvas, scene, settings);

    // Render the scene
    renderer.render();
}

run();