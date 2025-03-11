use raytracer::*;
mod parsing;

use std::io;
use std::sync::Arc;
use rayon::prelude::*;

 
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
 
    if let Some(hit_rec) = world.hit(r, 0.001, common::INFINITY) {
        if let Some(scatter_rec) = hit_rec.mat.scatter(r, &hit_rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
 
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn scene() -> HittableList {
    let mut world = HittableList::new();

    let glass = Arc::new(Dielectric::new(1.5));
    let copper = Arc::new(Metal::new(Color::new(0.72, 0.45, 0.20), 0.1));
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    // Plan plat
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Sphère
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass,
    )));

    // Cube
    world.add(Box::new(Cube::from_center(
        Point3::new(-0.5, 0.5, 1.0),
        1.0,
        copper.clone(),
    )));
    

    // Cylindre
    world.add(Box::new(Cylinder::new(
        Point3::new(2.0, 0.5, 0.0),
        0.5,
        1.0,
        copper.clone(),
        16,
    )));

    // Retourner la caméra modifiée avec les objets
    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;
 
    // World
 
    let world = scene();
 
    // Camera
    
    // Nouveau regard pour la caméra
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
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
    // Render
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = ((i as f64) + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = ((j as f64) + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
 
    eprint!("\nDone.\n");
}