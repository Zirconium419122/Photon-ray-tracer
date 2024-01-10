import './style.css'

// Get the canvas element and the 2d context
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

// Create ImageData object for direct pixel manipulation
const imageData = ctx.createImageData(canvas.width, canvas.height);
const data = imageData.data;

// Set the width and height of the canvas
canvas.width = 800;  // Replace 800 with your desired width
canvas.height = 600; // Replace 600 with your desired height



// Vector class
class Vector {
  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  // Method to add another vector
  add(v) {
    return new Vector(this.x + v.x, this.y + v.y, this.z + v.z);
  }

  // Method to subtract another vector
  subtract(v) {
    return new Vector(this.x - v.x, this.y - v.y, this.z - v.z);
  }

  // Method to multiply by a scalar
  multiply(scalar) {
    return new Vector(this.x * scalar, this.y * scalar, this.z * scalar);
  }

  // Method to divide by a scalar
  divide(scalar) {
    // Check for division by zero to avoid errors
    if (scalar !== 0) {
      return new Vector(this.x / scalar, this.y / scalar, this.z / scalar);
    } else {
      console.error("Division by zero!");
      return null;
    }
  }

  // Method to calculate the dot product with another vector
  dot(v) {
    return this.x * v.x + this.y * v.y + this.z * v.z;
  }

  // Method to calculate the cross product with another vector
  cross(v) {
    return new Vector(
      this.y * v.z - this.z * v.y,
      this.z * v.x - this.x * v.z,
      this.x * v.y - this.y * v.x
    );
  }

  // Method to calculate the magnitude of the vector
  magnitude() {
    return Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
  }

  // Method to normalize the vector (make it a unit vector)
  normalize() {
    const mag = this.magnitude();
    return new Vector(this.x / mag, this.y / mag, this.z / mag);
  }
}


// Utility class for vector and ray operations
class RayUtils {
  // Function to generate a random point on a sphere
  static randomPointOnSphere() {
    const theta = Math.random() * 2 * Math.PI;
    const phi = Math.acos(2 * Math.random() - 1);

    const x = Math.sin(phi) * Math.cos(theta);
    const y = Math.sin(phi) * Math.sin(theta);
    const z = Math.cos(phi);

    return new Vector(x, y, z)
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

      const pointInCube = new Vector(x, y, z);
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
  constructor(color, reflectionCoeff, emittedColor = new Vector(0, 0, 0), lightStrength = 0) {
    this.color = color;  // Surface color of the material
    this.reflectionCoeff = reflectionCoeff; // Reflection coefficient (0 for no reflection, 1 for full reflection)
    this.emittedColor = emittedColor; // Emitted light color
    this.lightStrength = lightStrength; // Light strength (intensity)
  }

  // Method to calculate the color of a point on the material
  shade(ray, intersectionPoint) {
    return this.color;
  }
}


// Ray class
class Ray {
  constructor(origin, direction) {
    this.origin = origin;       // Vector representing the ray's origin
    this.direction = direction; // Vector representing the ray's direction
  }

  // Metod to calculate the color by tracing the ray
  trace(state, x, y) {
    // Create seed for random number generator
    const numPixels = canvas.width * canvas.height;
    const pixelIndex = y * numPixels + x;
    state = pixelIndex * 485732;

    let incomingLight = new Vector(0, 0, 0);
    let rayColor = new Vector(1, 1, 1);

    let closestIntersection = null;

    // Recursivly reflect the ray
    for (let i = 0; i < maxReflectionDepth; i++) {
      // Test for intersections with objects in the scene
      for (const object of scene.objects) {
        const intersectionResult = object.intersect(this);

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
        const normal = object.calculateNormal(intersectionPoint);

        // Update the origin and direction of the ray for the next iteration
        this.origin = intersectionPoint;
        this.direction = RayUtils.RandomHemisphereDirection(normal, state);

        // Calculate the incoming light
        const material = object.material;
        const emittedLight = material.emittedColor.multiply(material.lightStrength);
        incomingLight = incomingLight.add(
          new Vector(
          emittedLight.x * rayColor.x,
          emittedLight.y * rayColor.y,
          emittedLight.z * rayColor.z
        ));
        rayColor = new Vector(rayColor.x * material.color.x, rayColor.y * material.color.y, rayColor.z * material.color.z);
      }

      // If the ray does not intersect with any object we exit the loop
      if (!closestIntersection) {
        break;
      }
    }

    return incomingLight;
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

  // Method to get the material color at a specific point on the sphere
  getColorAtPoint(point, ray) {
    const normal = this.calculateNormal(point);
    return this.material.shade(ray, { point, normal });
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
      return new Vector(Math.sign(dx), 0, 0);
    } else if (Math.abs(dy) > Math.abs(dz)) {
      // Point is on the face with the largest y-coordinate differance
      return new Vector(0, Math.sign(dy), 0);
    } else {
      // Point is on the face with the largest z-coordinate differance
      return new Vector(0, 0, Math.sign(dz));
    }
  }

  // Method to get the material color at a specific point on the cube
  getColorAtPoint(point, ray) {
    const normal = this.calculateNormal(point);
    return this.material.shade(ray, { point, normal });
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

  // Method to render the scene
  render() {
    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    // Create new ImageData object for direct pixel manipulation
    const imageData = ctx.createImageData(canvas.width, canvas.height);

    // Access the pixel data array (each pixel has 4 values: red, green, blue, and alpha)
    const data = imageData.data;

    let state = 367380976; // 37890367;
    const maxStateValue = 1e12; // Adjust as needed

    let cumulativeImageData = null;

    // Recursivly render the scene
    for (let frame = 0; frame < numFrames; frame++) {
      // Reset i to 0 at the start of each frame
      let i = 0;
      
      // Loop through each pixel on the canvas
      for (let y = 0; y < canvas.height; y++) {
        for (let x = 0; x < canvas.width; x++) {
          let accumulatedColor = new Vector(0, 0, 0);

          for (let sample = 0; sample < NumRayPerPixel; sample++) {
            // Create a ray from the camera to the current pixel
            const rayOrigin = new Vector(0, 0, 0);
            const aspectRatio = canvas.width / canvas.height;
            const rayDirection = new Vector(
              (x / canvas.width) * 2 - 1,
              ((y / canvas.height) * 2 - 1) / aspectRatio,
              -1
            ).normalize(); // Normalize the direction vector
            const ray = new Ray(rayOrigin, rayDirection);

            // Get the state for the number generator
            // state += (x + 349279) /** (x * 213574) * (y + 784674)*/ * (y * 426676);
            state = ((x + 349279) * (x * 213574) * (y + 784674) * (y * 426676) * (frame + 1)) % maxStateValue;


            // Trace the ray to get the color
            const color = ray.trace(state, x, y);

            // Accumulate the color
            accumulatedColor = accumulatedColor.add(color);
          }

          // Average the accumulated color over samples
          const averagedColor = accumulatedColor.divide(numSamples);

          // Set the pixel color in ImageData
          data[i] = averagedColor.x * 255;
          data[i + 1] = averagedColor.y * 255;
          data[i + 2] = averagedColor.z * 255;
          data[i + 3] = 255; // Alpha channel
          
          i += 4;
        }
      }

      // If it's not the first frame, average the pixel values
      if (cumulativeImageData) {
        // Average the pixel values over frames
        for (let i = 0; i < data.length; i++) {
          data[i] = Math.round((data[i] + cumulativeImageData.data[i]) / 2);
        }
      }

      // Put the modified ImageData back to the canvas
      ctx.putImageData(imageData, 0, 0);

      // Update the cumulativeImageData for the next frame
      cumulativeImageData = imageData;

      console.log(state);
    }
  }
}


//
const maxReflectionDepth = 5;
const NumRayPerPixel = 100;
const numFrames = 1;

// Add the light source 
const sphereCenter = new Vector(-5, -5, -10);
const sphereRadius = 5;
const sphereMaterial = new Material(
  new Vector(0, 0, 0),
  0,
  new Vector(1, 1, 1),
  20
);
const sphere = new Sphere(sphereCenter, sphereRadius, sphereMaterial)

console.log(sphere);

// Example usage
const sphereCenter1 = new Vector(0, 0, -5);
const sphereRadius1 = 1;
const sphereMaterial1 = new Material(new Vector(1, 0, 0));
const sphere1 = new Sphere(sphereCenter1, sphereRadius1, sphereMaterial1);

console.log(sphere1);

const sphereCenter2 = new Vector(3, 1, -11);
const sphereRadius2 = 1;
const sphereMaterial2 = new Material(new Vector(0, 1, 0));
const sphere2 = new Sphere(sphereCenter2, sphereRadius2, sphereMaterial2);

console.log(sphere2);

const sphereCenter3 = new Vector(0, 5, -5);
const sphereRadius3 = 4.5;
const sphereMaterial3 = new Material(new Vector(0.8, 0.8, 0.8));
const sphere3 = new Sphere(sphereCenter3, sphereRadius3, sphereMaterial3);

console.log(sphere3);

const cubeCenter = new Vector(-2, 1, -5);
const cubeSize = new Vector(1, 1, 1);
const cubeMaterial = new Material(new Vector(0, 0, 1));
const cube = new Cube(cubeCenter, cubeSize, cubeMaterial);

console.log(cube);

const scene = new Scene();
scene.addObject(sphere);
scene.addObject(sphere1);
scene.addObject(sphere2);
// scene.addObject(sphere3);
scene.addObject(cube);


// Render the scene
scene.render();
