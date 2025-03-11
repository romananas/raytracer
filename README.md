
# Raytracer (RT) - 3D Primitives Implementation

This project involves developing a minimalistic ray tracing engine capable of generating images by tracing rays through a 3D scene. The implementation supports multiple geometric primitives: quad, cube, sphere, and cylinder. Each object is defined with its physical properties (position, size, color) and interacts with lighting to produce realistic rendering. The goal is to deepen the understanding of ray tracing algorithms and 3D intersection calculations.

It can be used has a standelone programm and has a crate
## Installation

from source
```bash
  git clone https://github.com/romananas/raytracer
  cd raytracer

  cargo build --release
  ./target/release/raytracer > img.ppm
```

## Exemples of results

Exemple used in The Ray Tracing Road to Rust

![Result of the programm computer generated image](https://github.com/romananas/raytracer/blob/main/img/exemple.png)

Scene asked in z01 exercise RT

![Result of the programm computer generated image](https://github.com/romananas/raytracer/blob/main/img/final_scene.png)

## Acknowledgements

 - [The Joy of building a raytracer in rust](https://blog.singleton.io/posts/2022-01-02-raytracing-with-rust/)
 - [The Ray Tracing Road to Rust](https://the-ray-tracing-road-to-rust.vercel.app/)
 - [Ray Tracing in One Weekend](https://raytracing.github.io/)

