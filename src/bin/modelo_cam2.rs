use macroquad::{
    camera::Camera3D,
    input::{is_mouse_button_down, mouse_delta_position, mouse_wheel},
    math::Vec3,
    prelude::*,
    window::next_frame,
};

#[macroquad::main("Segundo Modelo de Camera")]
async fn main() {
    let mut theta: f32 = 0.;
    let mut phi: f32 = 2.;
    let mut rho: f32 = 800.;

    loop {
        clear_background(BLACK);
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_delta = mouse_delta_position();
            theta += mouse_delta.x * 5.;
            phi += mouse_delta.y * 5.;
        }

        let scroll = mouse_wheel().1;
        rho *= 1.0 - scroll * 0.1;

        phi = phi.clamp(0.1, std::f32::consts::PI - 0.1);

        let r = rho * phi.sin();

        let (x,y,z) = (r*theta.cos(), r*theta.sin(), rho*phi.cos());

        
        let camera = Camera3D {
            position: Vec3::new(x,y,z),
            target: Vec3::ZERO,
            up: Vec3::Z,
            ..Default::default()
        };

        set_camera(&camera);

        draw_sphere(Vec3::ZERO, 100., None, RED);
        draw_sphere(vec3(0., 0., 500.), 100., None, BLUE);
        draw_cube(vec3(0., 400., 0.), vec3(100., 100., 100.), None, GRAY);
        set_default_camera();
        println!(
            "Câmera: x={:.0}, y={:.0}, z={:.0}",
            camera.position.x, camera.position.y, camera.position.z
        );
        println!(
            "phi: {:.0}, rho{:.0}, theta{:.0}",
            phi,rho, theta
        );
        next_frame().await;
    }
}
