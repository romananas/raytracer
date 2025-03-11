
# Raytracer (RT) - 3D Primitives Implementation

This project involves developing a minimalistic ray tracing engine capable of generating images by tracing rays through a 3D scene. The implementation supports multiple geometric primitives: quad, cube, sphere, and cylinder. Each object is defined with its physical properties (position, size, color) and interacts with lighting to produce realistic rendering. The goal is to deepen the understanding of ray tracing algorithms and 3D intersection calculations.
## Installation

from source
```bash
  git clone https://github.com/romananas/raytracer
  cd raytracer
  bash build.sh
```
    
## Usage/Examples

Placing the camera in space
```rust
    use raytracer::Camera;
    ...

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 15.0;
    let aperture = 0.1;
 
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
```

Creating a material
```rust
    let glass = Arc::new(Dielectric::new(1.5));
```

Placing a glass sphere
```rust
world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.5, -1.0), // Position of the sphere
        0.5,                         // it size
        glass, // the previous material
    )));
```