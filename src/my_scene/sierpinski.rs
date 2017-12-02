#![allow(unused_imports)]

use geometry::prim::Prim;
use geometry::prims::{Plane, Sphere, Triangle, TriangleOptions};
use light::light::Light;
use light::lights::{PointLight, SphereLight};
use material::materials::{CookTorranceMaterial, FlatMaterial, PhongMaterial};
use material::Texture;
use material::textures::{CheckerTexture, CubeMap, UVTexture, ImageTexture};
use raytracer::animator::CameraKeyframe;
use raytracer::compositor::ColorRGBA;
use scene::{Camera, Scene};
use vec3::Vec3;
use mat4::Mat4;


pub fn get_camera(image_width: u32, image_height: u32, fov: f64) -> Camera {
    Camera::new(
        Vec3 { x: 0.0, y: 0.0, z: -4.0 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        fov,
        image_width,
        image_height
    )
}

pub fn get_scene() -> Scene {
    let shiny = CookTorranceMaterial { k_a: 0.0, k_d: 0.2, k_s: 1.0, k_sg: 0.8, k_tg: 0.0, gauss_constant: 5.0, roughness: 0.01, glossiness: 0.0, ior: 0.25, ambient: Vec3::one(), diffuse: Vec3 { x: 1.0, y: 1.0, z: 1.0 }, specular: Vec3 { x: 0.9, y: 0.9, z: 0.9 }, transmission: Vec3::zero(), diffuse_texture: None };

    let max = 5;
    let mut pts = vec![Vec3 { x: 0.5, y: 0.28867513, z: 0.20412415 }];
    let r = 1.0 / 2.0;
    let t1 = Mat4::scale_about_point(&Vec3 { x: 0.0, y: 0.0, z: 0.0 }, r);
    let t2 = Mat4::scale_about_point(&Vec3 { x: 1.0, y: 0.0, z: 0.0 }, r);
    let t3 = Mat4::scale_about_point(&Vec3 { x: 0.5, y: 3.0f64.powf(0.5) / 2.0, z: 0.0 }, r);
    let t4 = Mat4::scale_about_point(&Vec3 {
        x: 0.5,
        y: 1.0 / 3.0 * 3.0f64.powf(0.5) / 2.0,
        z: ((3.0f64.powf(0.5) / 2.0).powi(2) - (1.0 / 3.0 * 3.0f64.powf(0.5) / 2.0).powi(2)).powf(0.5)
    }, r);
    let model_view = Mat4::rotate_z_deg_matrix(180.0) *
        Mat4::rotate_x_deg_matrix(180.0 / 8.0) *
        Mat4::rotate_x_deg_matrix(180.0) *
        Mat4::new(2.0, 0.0, 0.0, -1.0,
                  0.0, 2.0, 0.0, -1.0,
                  0.0, 0.0, 2.0, -1.0,
                  0.0, 0.0, 0.0, 1.0);


    let mut lights: Vec<Box<Light + Send + Sync>> = Vec::new();
    lights.push(Box::new(SphereLight { position: Vec3 { x: -1.0, y: 1.0, z: -2.0 }, color: Vec3::one(), radius: 1.0 }));

    let mut prims: Vec<Box<Prim + Send + Sync>> = Vec::new();
    for i in 0..max {
        let radius = 0.5f64.powi(i) / 24f64.sqrt();
        let mut new_pts = pts.clone();
        {
            new_pts.extend(pts.iter().cloned().map(|x| Mat4::mult_p(&t1, &x)));
            new_pts.extend(pts.iter().cloned().map(|x| Mat4::mult_p(&t2, &x)));
            new_pts.extend(pts.iter().cloned().map(|x| Mat4::mult_p(&t3, &x)));
            new_pts.extend(pts.iter().cloned().map(|x| Mat4::mult_p(&t4, &x)));
        }
        for p in pts {
            prims.push(Box::new(
                Sphere {
                    center: Mat4::mult_p(&model_view, &p),
                    radius: radius * 2.0,
                    material: Box::new(shiny.clone())
                }));
        }
        pts = new_pts;
    }

    let octree = prims.into_iter().collect();
    Scene {
        lights: lights,
        octree: octree,
        background: Vec3::zero(),
        skybox: None
    }
}

pub struct SierpinskiConfig;

impl super::SceneConfig for SierpinskiConfig {
    fn get_camera(&self, image_width: u32, image_height: u32, fov: f64) -> Camera {
        get_camera(image_width, image_height, fov)
    }

    fn get_scene(&self) -> Scene {
        get_scene()
    }
}