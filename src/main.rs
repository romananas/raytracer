use raytracer::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use rayon::prelude::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 600;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 25;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
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

fn render_scene(filename: &str, scene: HittableList, cam: Camera) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rRendering {} | Scanlines remaining: {} ", filename, j);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = ((i as f64) + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = ((j as f64) + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &scene, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();

        for color in pixel_colors {
            color::write_color(&mut writer, color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\n{} Done.", filename);
}

fn scene1_sphere() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let sphere = Arc::new(Lambertian::new(Color::new(0.9, 0.2, 0.3)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.5, 0.0), 0.5, sphere)));

    let cam = Camera::new(
        Point3::new(2.0, 1.0, 2.0),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        40.0,
        ASPECT_RATIO,
        0.0,
        1.0,
    );

    (world, cam)
}

fn scene2_plane_cube() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Color::new(0.4, 0.4, 0.4)));
    let cube = Arc::new(Metal::new(Color::new(0.2, 0.2, 0.2), 0.3));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground)));
    world.add(Box::new(Cube::from_center(Point3::new(0.0, 0.5, 0.0), 1.0, cube)));

    let cam = Camera::new(
        Point3::new(2.0, 1.5, 2.0),
        Point3::new(0.0, 0.5, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        40.0,
        ASPECT_RATIO,
        0.0,
        1.0,
    );

    (world, cam)
}

fn scene3_all_objects() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let metal = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let glass = Arc::new(Dielectric::new(1.5));
    let matte = Arc::new(Lambertian::new(Color::new(0.2, 0.4, 0.8)));

    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.5, 0.0), 0.5, glass)));
    world.add(Box::new(Cube::from_center(Point3::new(1.0, 0.5, 0.0), 1.0, metal.clone())));
    world.add(Box::new(Cylinder::new(Point3::new(0.0, 0.5, 1.0), 0.3, 1.0, matte, 32)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground)));

    let cam = Camera::new(
        Point3::new(5.0, 3.0, 5.0),
        Point3::new(0.0, 0.5, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        30.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    (world, cam)
}

fn scene4_all_objects_alt_cam() -> (HittableList, Camera) {
    let (world, _) = scene3_all_objects();

    let cam = Camera::new(
        Point3::new(-3.0, 4.0, 3.0),
        Point3::new(0.0, 0.5, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        30.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    (world, cam)
}

fn main() {
    let scenes = [
        ("scene1.ppm", scene1_sphere()),
        ("scene2.ppm", scene2_plane_cube()),
        ("scene3.ppm", scene3_all_objects()),
        ("scene4.ppm", scene4_all_objects_alt_cam()),
    ];

    for (filename, (scene, cam)) in scenes {
        render_scene(filename, scene, cam);
    }
}
