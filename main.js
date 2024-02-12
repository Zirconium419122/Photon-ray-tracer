import './style.css'
import init, * as wasm from "./pkg/raytracer.js"

await init();

// Get the canvas element
const canvas = document.getElementById('canvas');

// Set the width and height of the canvas
canvas.width = 800;  // Replace 800 with your desired width
canvas.height = 600; // Replace 600 with your desired height

async function run() {
    // Define the settigns of the renderer
    const maxReflectionDepth = 10;
    const numSamples = 5;
    const numFrames = 1;
    const settings = new wasm.Settings(maxReflectionDepth, numSamples, numFrames);

    // Create the scene
    const scene = new wasm.Scene();

    // Add the light source 
    {
      const sphereCenter = new wasm.Vector(-5, -5, -10);
      const sphereRadius = 5;
      const sphereMaterial = new wasm.Material(
        new wasm.Vector(0, 0, 0),
        1,
        new wasm.Vector(1, 1, 1),
        2
      );
      const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial);

      scene.add_sphere(sphere);
      console.log(sphere);
    }
    
    // Add the rest of the objects
    {
      const sphereCenter = new wasm.Vector(0, 0, -5);
      const sphereRadius = 1;
      const sphereMaterial = new wasm.Material(new wasm.Vector(1, 0, 0), 1, new wasm.Vector(0, 0, 0), 0);
      const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial);
  
      scene.add_sphere(sphere);
      console.log(sphere);      
    }

    {
      const sphereCenter = new wasm.Vector(3, 1, -10);
      const sphereRadius = 1;
      const sphereMaterial = new wasm.Material(new wasm.Vector(0, 1, 0), 1, new wasm.Vector(0, 0, 0), 0);
      const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial);
  
      scene.add_sphere(sphere);
      console.log(sphere);      
    }

    {
      const sphereCenter = new wasm.Vector(0, 5, -5);
      const sphereRadius = 4.5;
      const sphereMaterial = new wasm.Material(new wasm.Vector(0.5, 0.5, 0.5), 1, new wasm.Vector(0, 0, 0), 0);
      const sphere = new wasm.Sphere(sphereCenter, sphereRadius, sphereMaterial);
  
      scene.add_sphere(sphere);
      console.log(sphere);      
    }

    {
      const cubeCenter = new wasm.Vector(-2, 1, -5);
      const cubeSize = new wasm.Vector(1, 1, 1);
      const cubeMaterial = new wasm.Material(new wasm.Vector(0, 0, 1), 1, new wasm.Vector(0, 0, 0), 0);
      const cube = new wasm.Cube(cubeCenter, cubeSize, cubeMaterial);

      scene.add_cube(cube);
      console.log(cube);
    }

    // Create the renderer
    const renderer = new wasm.Renderer(canvas, scene, settings);

    // Render the scene
    renderer.render();
}

run();