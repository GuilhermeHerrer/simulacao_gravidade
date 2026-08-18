use core::f32;

use macroquad::{
    input::KeyCode::{A, D, E, Q, S, W},
    prelude::*,
};

fn desenhar_grid(espacamento: f32, comprimento: f32) {
    draw_line_3d(vec3(0., comprimento, 0.), vec3(0., -comprimento, 0.), WHITE);
    draw_line_3d(vec3(comprimento, 0., 0.), vec3(-comprimento, 0., 0.), WHITE);

    let quantidade = (comprimento / espacamento) as i32;

    for n in 0..=quantidade {
        let distancia = n as f32 * espacamento;

        draw_line_3d(vec3(distancia, comprimento, 0.), vec3(distancia, -comprimento, 0.), WHITE);
        draw_line_3d(vec3(comprimento, distancia, 0.), vec3(-comprimento, distancia, 0.), WHITE);
        draw_line_3d(vec3(-distancia, comprimento, 0.), vec3(-distancia, -comprimento, 0.), WHITE);
        draw_line_3d(vec3(comprimento, -distancia, 0.), vec3(-comprimento, -distancia, 0.), WHITE);
    }
}

#[macroquad::main("Grid sem nodes")]
async fn main() {
    let mut theta: f32 = 0.;
    let mut phi: f32 = 0.5;
    let mut rho: f32 = 500.;

    loop {
        clear_background(BLACK);

        if is_key_down(A) {
            theta += 0.05;
        }
        if is_key_down(D) {
            theta -= 0.05;
        }
        if is_key_down(W) {
            phi += 0.05;
        }
        if is_key_down(S) {
            phi -= 0.05;
        }
        if is_key_down(Q) {
            rho += 5.;
        }
        if is_key_down(E) {
            rho -= 5.;
        }

        phi = phi.clamp(0.1, std::f32::consts::PI - 0.1);

        let r = rho * phi.sin();

        let camera = Camera3D {
            position: Vec3::new(r * theta.cos(), r * theta.sin(), rho * phi.cos()),
            target: Vec3::ZERO,
            up: Vec3::Z,
            ..Default::default()
        };

        set_camera(&camera);

        desenhar_grid(50., 1000.);

        draw_sphere(Vec3::ZERO, 100., None, RED);
        draw_sphere(vec3(0., 0., 500.), 100., None, BLUE);
        set_default_camera();
        println!(
            "Câmera: x={:.0}, y={:.0}, z={:.0}",
            camera.position.x, camera.position.y, camera.position.z
        );
        next_frame().await;
    }
}
