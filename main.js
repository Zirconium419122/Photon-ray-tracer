import './style.css'
import init, * as wasm from "./pkg/raytracer.js"

await init();

const TraceRayVectorPool = new wasm.VectorPool(10);

// Get the canvas element and the 2d context
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

// Create ImageData object for direct pixel manipulation
const imageData = ctx.createImageData(canvas.width, canvas.height);
const data = imageData.data;

// Set the width and height of the canvas
canvas.width = 400;  // Replace 800 with your desired width
canvas.height = 300; // Replace 600 with your desired height

// Get background color
function getBackgroundColor(ray) {
  // Map the vertical position of the ray to a gradient color
  const t = 0.5 * (ray.direction.y + 1.0);
  
  // Linear gradient from white to blue
  const white = new wasm.Vector(1, 1, 1);
  const blue = new wasm.Vector(0.5, 0.7, 1.0);

  const gradient = white.multiply(1.0 - t).add(blue.multiply(t))

  white.free();
  blue.free();

  return gradient;
}

// Utility class for vector and ray operations
class Utils {
  // Function to generate a random point on a sphere
  static randomPointOnSphere() {
    const theta = Math.random() * 2 * Math.PI;
    const phi = Math.acos(2 * Math.random() - 1);

    const x = Math.sin(phi) * Math.cos(theta);
    const y = Math.sin(phi) * Math.sin(theta);
    const z = Math.cos(phi);

    return new wasm.Vector(x, y, z)
  }

  // PCG (permuated congruentila generator). Thanks to:
  // www.pcg-random.org and www.shadertoy.com/view/XlGcRh
  static RandomValue(state) {
    state = state * 747796405 + 2891336453;
    let result = ((state >> ((state >> 28) + 4)) ^ state) * 277803737;
    result = (result >> 22) ^ result;
    return result / 4294967295.0;
  }

  static RandomDirection(state) {
    for (let limit = 0; limit < 100; limit++) {
      const x = this.RandomValue(state) * 2 - 1;
      const y = this.RandomValue(state += 13146368) * 2 - 1;
      const z = this.RandomValue(state += 23652568) * 2 - 1;

      const pointInCube = new wasm.Vector(x, y, z);
      const sqrDstFromCenter = pointInCube.dot(pointInCube);

      // If point is inside sphere, scale it to lie on the surface (otherwise, keep trying)
      if (sqrDstFromCenter <= 1) {
        return pointInCube.divide(Math.sqrt(sqrDstFromCenter));
      }
    }

    return 0;
  }

  static RandomHemisphereDirection(normal, state) {
    const direction = this.RandomDirection(state);
    return direction.multiply(Math.sign(normal.dot(direction)));
  }
}


// Material class
class Material {
  constructor(color, reflectionCoeff, emittedColor = new wasm.Vector(0, 0, 0), lightStrength = 0) {
    this.color = color;  // Surface color of the material
    this.reflectionCoeff = reflectionCoeff; // Reflection coefficient (0 for no reflection, 1 for full reflection)
    this.emittedColor = emittedColor; // Emitted light color
    this.lightStrength = lightStrength; // Light strength (intensity)
  }
}


// Ray class
class Ray {
  constructor(origin, direction) {
    this.origin = origin;       // Vector representing the ray's origin
    this.direction = direction; // Vector representing the ray's direction
  }

  reflect(normal) {
    const reflectedDirection = RandomHemishereDirection(normal, this.state);
    return new this.constructor(this.origin, reflectedDirection);
  }

  // Function to get a point along the ray given a parameter t
  pointAtParameter(t) {
    return this.origin.add(this.direction.multiply(t));
  }
}


// Sphere class
class Sphere {
  constructor(center, radius, material) {
    this.center = center;     // Vector representing the center of the sphere
    this.radius = radius;     // Radius of the sphere
    this.material = material; // Material of the sphere
  }

  // Method to test if a ray intersects with the sphere
  intersect(ray) {
    const oc = ray.origin.subtract(this.center);
    const a = ray.direction.dot(ray.direction);
    const b = oc.dot(ray.direction) * 2;
    const c = oc.dot(oc) - this.radius * this.radius;
    const discriminant = b * b - 4 * a * c;

    if (discriminant >= 0) {
      // Ray intersects the sphere, calculate intersection point
      const t1 = (-b - Math.sqrt(discriminant)) / (2 * a);
      const t2 = (-b + Math.sqrt(discriminant)) / (2 * a);

      // Return the smaller positive intersection point
      const t = Math.min(t1, t2);

      if (t > 0) {
        const intersectionPoint = ray.pointAtParameter(t);
        return { t, intersectionPoint, intersectionObject: this };
      }
    }

    // Ray does not intersect the sphere
    return null;
  }

  // Method to calculate the normal at a point on the sphere
  calculateNormal(point) {
    return point.subtract(this.center).normalize();
  }
}

// Cube class
class Cube {
  constructor(center, size, material) {
    this.center = center;     // Vector representing the center of the cube
    this.size = size;         // Vector representing the lengths of the three sides (x, y, z)
    this.material = material; // Material of the cube
  }

  // Method to test if a ray intersects with the cube
  intersect(ray) {
    const halfSize = this.size.multiply(0.5);

    // Calculate the minimum and maximum extents along each axis
    const minX = this.center.x - halfSize.x;
    const minY = this.center.y - halfSize.y;
    const minZ = this.center.z - halfSize.z;

    const maxX = this.center.x + halfSize.x;
    const maxY = this.center.y + halfSize.y;
    const maxZ = this.center.z + halfSize.z;

    // Calculate the intersection distances along each axis
    const tMinX = (minX - ray.origin.x) / ray.direction.x;
    const tMaxX = (maxX - ray.origin.x) / ray.direction.x;

    const tMinY = (minY - ray.origin.y) / ray.direction.y;
    const tMaxY = (maxY - ray.origin.y) / ray.direction.y;

    const tMinZ = (minZ - ray.origin.z) / ray.direction.z;
    const tMaxZ = (maxZ - ray.origin.z) / ray.direction.z;

    // Find the intersection intervals along each axis
    const tMin = Math.max(Math.max(Math.min(tMinX, tMaxX), Math.min(tMinY, tMaxY)), Math.min(tMinZ, tMaxZ));
    const tMax = Math.min(Math.min(Math.max(tMinX, tMaxX), Math.max(tMinY, tMaxY)), Math.max(tMinZ, tMaxZ));

    // Check if there is a valid intersection
    if (tMin <= tMax && tMax > 0) {
      // Return the intersection point at the minmum distance
      const intersectionPoint = ray.pointAtParameter(tMin);
      return { t: tMin, intersectionPoint, intersectionObject: this };
    }

    // Ray does not intersect with the cube
    return null;
  }

  // Method to calculate the normal at a point on the cube
  calculateNormal(point) {
    const halfSize = this.size.multiply(0.5);

    // Calculate the differences between the point's coordinates and the cube's center
    const dx = point.x - this.center.x;
    const dy = point.y - this.center.y;
    const dz = point.z - this.center.z;

    // Identify the face closest to the point and assign the normal accordingly
    if (Math.abs(dx) > Math.abs(dy) && Math.abs(dx) > Math.abs(dz)) {
      // Point is on the face with the largest x-coordinate differance
      return new wasm.Vector(Math.sign(dx), 0, 0);
    } else if (Math.abs(dy) > Math.abs(dz)) {
      // Point is on the face with the largest y-coordinate differance
      return new wasm.Vector(0, Math.sign(dy), 0);
    } else {
      // Point is on the face with the largest z-coordinate differance
      return new wasm.Vector(0, 0, Math.sign(dz));
    }
  }
}

// Renderer class
class Renderer {
  constructor(canvas, scene) {
    this.canvas = canvas;
    this.scene = scene;
  }

  // Method to render the scene
  Render() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    // Create new ImageData object for direct pixel manipulation
    const imageData = ctx.createImageData(canvas.width, canvas.height);

    // Access the pixel data array (each pixel has 4 values: red, green, blue, and alpha)
    const data = imageData.data;

    let state = 367380976; // 37890367;
    const maxStateValue = 1e9; // Adjust as needed

    let cumulativeImageData = imageData;

    // Recursivly render the scene
    for (let frame = 0; frame < numFrames; frame++) {
      // Reset i to 0 at the start of each frame
      let i = 0;
      
      // Loop through each pixel on the canvas
      for (let y = 0; y < canvas.height; y++) {
        for (let x = 0; x < canvas.width; x++) {
          // Get the state for the number generator
          state = ((x + 349279) * (x * 213574) * (y + 784674) * (y * 426676) * (frame + 1)) % maxStateValue;

          // Call the PerPixel method to get the color at the pixel
          let color = this.PerPixel(x, y, state);

          // Set the pixel color in ImageData
          data[i] = color.x * 255;
          data[i + 1] = color.y * 255;
          data[i + 2] = color.z * 255;
          data[i + 3] = 255; // Alpha channel
          
          i += 4;
        }
        
        console.log(`Row number ${y} is complete`);
      }

      // Update the cumulativeImageData with averaging the pixel values over frames
      for (let i = 0; i < data.length; i++) {
        cumulativeImageData[i] = cumulativeImageData + (data[i] / numFrames);
      }

      console.log(`Frame: ${frame} ended with this state: ${state}`);
    }

    // Put the modified ImageData back to the canvas
    ctx.putImageData(cumulativeImageData, 0, 0);
  }

  // Method to render each pixel
  PerPixel(x, y, state) {
    // Make the accumulateColor variable
    let accumulatedColor = new wasm.Vector(0, 0, 0);

    for (let sample = 0; sample < numSamples; sample++) {
      // Calculate jittered sample position within the pixel
      const jitterX = (Math.random() - 0.5) / 2;
      const jitterY = (Math.random() - 0.5) / 2;

      // Calculate pixel coordinates for the jittered sample
      const sampleX = x + (sample + jitterX) / numSamples;
      const sampleY = y + (sample + jitterY) / numSamples;
      
      // Create a ray from the camera to the current pixel
      const rayOrigin = new wasm.Vector(0, 0, 0);
      const aspectRatio = canvas.width / canvas.height;
      const rayDirection = new wasm.Vector(
        (sampleX / canvas.width) * 2 - 1,
        ((sampleY / canvas.height) * 2 - 1) / aspectRatio,
        -1
      ).normalize(); // Normalize the direction vector
      const ray = new Ray(rayOrigin, rayDirection);

      // Get the state for the number generator
      state = Utils.RandomValue(sample * (sample + 568) * (sample + 234) * (sample + 345) * (sample + 123));

      // Trace the ray to get the color
      const color = this.TraceRay(ray, sampleX, sampleY, state);

      // Accumulate the color
      accumulatedColor = accumulatedColor.add(color);
    }

    // Average the accumulated color over samples and return
    return accumulatedColor.divide(numSamples);
  }

  // Metod to calculate the color by tracing the ray
  TraceRay(ray, x, y, state) {
    // Create seed for random number generator
    const numPixels = canvas.width * canvas.height;
    const pixelIndex = y * numPixels + x;
    state += pixelIndex * 485732;

    TraceRayVectorPool.set_values(0, 0, 0, 0);
    let incomingLight = TraceRayVectorPool.get(0);
    TraceRayVectorPool.set_values(1, 1, 1, 1);
    let rayColor = TraceRayVectorPool.get(1);

    let closestIntersection = null;

    // Recursivly reflect the ray
    for (let i = 0; i < maxReflectionDepth; i++) {
      // // Change the state every reflection
      // state += 243723;

      // Test for intersections with objects in the scene
      for (const object of this.scene.objects) {
        const intersectionResult = object.intersect(ray);

        if (intersectionResult) {
          if (
            !closestIntersection ||
            intersectionResult.t < closestIntersection.t
          ) {
            closestIntersection = intersectionResult;
          }
        }
      }

      if (closestIntersection) {
        const intersectionPoint = closestIntersection.intersectionPoint;
        const object = closestIntersection.intersectionObject;

        // Get the normal on the object
        TraceRayVectorPool.set(2, object.calculateNormal(intersectionPoint));
        const normal = TraceRayVectorPool.get(2);

        // Update the origin and direction of the ray for the next iteration
        ray.origin = intersectionPoint;
        ray.direction = Utils.RandomHemisphereDirection(normal, state);

        // Calculate the incoming light
        const material = object.material;
        TraceRayVectorPool.set(3, material.emittedColor.multiply(material.lightStrength));
        const emittedLight = TraceRayVectorPool.get(3);
        TraceRayVectorPool.set_values(4, emittedLight.x * rayColor.x, emittedLight.y * rayColor.y, emittedLight.z * rayColor.z);
        const emission = TraceRayVectorPool.get(4);
        incomingLight = incomingLight.add(emission);
        TraceRayVectorPool.set_values(1, rayColor.x * material.color.x, rayColor.y * material.color.y, rayColor.z * material.color.z);

        if (object.material.lightStrength > 0) {
          return incomingLight;
        }
      }

      // If no intersection, return background color
      if (!closestIntersection) {
        let BackgroundColor = getBackgroundColor(ray);
        return new wasm.Vector(rayColor.x * BackgroundColor.x, rayColor.y * BackgroundColor.y, rayColor.z * BackgroundColor.z);
      }

      closestIntersection = null;
    }

    return incomingLight;
  }
}

// Scene class
class Scene {
  constructor() {
    this.objects = []; // Array to store the objects in the scene
  }

  // Method to add objects to the scene
  addObject(object) {
    this.objects.push(object);
  }
}


// Define the settigns of the renderer
const maxReflectionDepth = 10;
const numSamples = 1;
const numFrames = 1;

// Add the light source 
const sphereCenter = new wasm.Vector(-5, -5, -10);
const sphereRadius = 5;
const sphereMaterial = new Material(
  new wasm.Vector(0, 0, 0),
  0,
  new wasm.Vector(1, 1, 1),
  2
);
const sphere = new Sphere(sphereCenter, sphereRadius, sphereMaterial)

console.log(sphere);

// Example usage
const sphereCenter1 = new wasm.Vector(0, 0, -5);
const sphereRadius1 = 1;
const sphereMaterial1 = new Material(new wasm.Vector(1, 0, 0));
const sphere1 = new Sphere(sphereCenter1, sphereRadius1, sphereMaterial1);

console.log(sphere1);

const sphereCenter2 = new wasm.Vector(3, 1, -11);
const sphereRadius2 = 1;
const sphereMaterial2 = new Material(new wasm.Vector(0, 1, 0));
const sphere2 = new Sphere(sphereCenter2, sphereRadius2, sphereMaterial2);

console.log(sphere2);

const sphereCenter3 = new wasm.Vector(0, 5, -5);
const sphereRadius3 = 4.5;
const sphereMaterial3 = new Material(new wasm.Vector(0.5, 0.5, 0.5));
const sphere3 = new Sphere(sphereCenter3, sphereRadius3, sphereMaterial3);

console.log(sphere3);

const cubeCenter = new wasm.Vector(-2, 1, -5);
const cubeSize = new wasm.Vector(1, 1, 1);
const cubeMaterial = new Material(new wasm.Vector(0, 0, 1));
const cube = new Cube(cubeCenter, cubeSize, cubeMaterial);

console.log(cube);

const scene = new Scene();
scene.addObject(sphere);
scene.addObject(sphere1);
scene.addObject(sphere2);
scene.addObject(sphere3);
scene.addObject(cube);

// Create the renderer
const renderer = new Renderer(canvas, scene);

// Render the scene
renderer.Render()
